use crossbeam_channel::Sender;
use log::warn;

pub(crate) struct BatchSender<T>
where
    T: Clone,
{
    buffer: Vec<T>,
    tx: Sender<Vec<T>>,
    limit: usize,
}

impl<T> BatchSender<T>
where
    T: Clone,
{
    pub(crate) fn new(tx: Sender<Vec<T>>, limit: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(limit),
            tx,
            limit,
        }
    }

    pub(crate) fn send(&mut self, item: T) -> bool {
        self.buffer.push(item);
        if self.needs_flush() {
            return self.send_buffer();
        };
        true
    }

    fn needs_flush(&self) -> bool {
        self.buffer.len() >= self.limit
    }

    fn send_buffer(&mut self) -> bool {
        if let Err(err) = self.tx.send(self.buffer.clone()) {
            warn!("{err}");
            return false;
        }
        self.buffer = Vec::with_capacity(self.limit);
        true
    }
}

impl<T> Drop for BatchSender<T>
where
    T: Clone,
{
    fn drop(&mut self) {
        if !self.buffer.is_empty() && !self.send_buffer() {
            println!("Failed to clear buffer during drop");
        }
    }
}
