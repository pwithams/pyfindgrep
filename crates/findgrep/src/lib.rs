mod batch;
mod data;

use crate::batch::BatchSender;
use crate::data::RegexMatcherInfo;
pub use crate::data::{FindResult, GrepResult};

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender, unbounded};
use grep_matcher::Matcher;
use grep_regex::RegexMatcher;
use grep_searcher::Searcher;
use grep_searcher::sinks::UTF8;
use ignore::{WalkBuilder, WalkState};
use log::warn;
use regex::bytes::Regex;
use std::path::Path;

pub(crate) fn path_is_match(path: &Path, regex_patterns: &[Regex]) -> bool {
    regex_patterns.is_empty()
        || regex_patterns
            .iter()
            .any(|pat| pat.is_match(path.to_str().unwrap().as_bytes()))
}

pub(crate) fn grep_file(path: &Path, matcher_info: &RegexMatcherInfo) -> Vec<GrepResult> {
    let mut results: Vec<GrepResult> = Vec::new();
    let search_status = Searcher::new().search_path(
        &matcher_info.matcher,
        path,
        UTF8(|lnum, line| {
            if let Ok(Some(result)) = matcher_info.matcher.find(line.as_bytes()) {
                results.push(GrepResult {
                    pattern: matcher_info.pattern.clone(),
                    matching_line: line.to_string(),
                    matching_text: line[result].to_string(),
                    lineno: lnum,
                });
            };
            Ok(true)
        }),
    );
    if let Err(err) = search_status {
        warn!("{err}");
    }
    results
}

pub struct FindGrepConfig {
    pub threads: usize,
    pub ignore_hidden: bool,
    pub buffer_size: usize,
    pub log_errors: bool,
    pub only_files: bool,
    pub find_patterns: Vec<String>,
    pub grep_patterns: Vec<String>,
}

impl Default for FindGrepConfig {
    fn default() -> FindGrepConfig {
        FindGrepConfig {
            threads: 0,
            ignore_hidden: true,
            buffer_size: 1024,
            log_errors: false,
            only_files: true,
            find_patterns: vec![],
            grep_patterns: vec![],
        }
    }
}

pub fn findgrep(path: String, config: FindGrepConfig) -> Result<Vec<FindResult>> {
    // parse and validate regex patterns
    let mut find_regex_patterns = Vec::new();
    for item in config.find_patterns {
        match Regex::new(&item) {
            Ok(regex) => {
                find_regex_patterns.push(regex);
                continue;
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }
    let mut grep_regex_patterns = Vec::new();
    for item in config.grep_patterns {
        match RegexMatcher::new(&item) {
            Ok(regex) => {
                grep_regex_patterns.push(RegexMatcherInfo {
                    matcher: regex,
                    pattern: item.to_string(),
                });
                continue;
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    let walker = WalkBuilder::new(path)
        .hidden(config.ignore_hidden)
        .threads(config.threads)
        .build_parallel();

    let (tx, rx): (Sender<Vec<FindResult>>, Receiver<Vec<FindResult>>) = unbounded();
    walker.run(|| {
        let mut batch_sender = BatchSender::new(tx.clone(), config.buffer_size);
        let worker_find_patterns = &find_regex_patterns;
        let worker_grep_patterns = &grep_regex_patterns;
        Box::new(move |entry_result| {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        if worker_find_patterns.is_empty()
                            || path_is_match(path, worker_find_patterns)
                        {
                            let mut grep_results = Vec::new();
                            for matcher_info in worker_grep_patterns {
                                grep_results.extend(grep_file(path, matcher_info));
                            }
                            if !grep_results.is_empty() || worker_grep_patterns.is_empty() {
                                batch_sender.send(FindResult {
                                    path: path.to_path_buf(),
                                    path_type: "file".to_string(),
                                    grep_results,
                                });
                            }
                        }
                    } else if !config.only_files && path_is_match(path, worker_find_patterns) {
                        batch_sender.send(FindResult {
                            path: path.to_path_buf(),
                            path_type: "directory".to_string(),
                            grep_results: Vec::new(),
                        });
                    }
                }
                Err(err) => {
                    if config.log_errors {
                        println!("Error: {err}");
                    }
                }
            };
            WalkState::Continue
        })
    });

    drop(tx);

    let mut find_results = Vec::new();
    for find_result_vec in rx {
        for find_result in find_result_vec {
            find_results.push(find_result);
        }
    }
    Ok(find_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_is_match_single_pattern() {
        let path = Path::new("/tmp/some/path.txt");
        let pattern = Regex::new(".*.txt").unwrap();
        let patterns = vec![pattern];
        let is_match = path_is_match(path, &patterns);
        assert!(is_match);
    }

    #[test]
    fn path_is_match_no_patterns() {
        let path = Path::new("/tmp/some/path.txt");
        let patterns = vec![];
        let is_match = path_is_match(path, &patterns);
        assert!(is_match);
    }

    #[test]
    fn path_is_match_pattern_miss() {
        let path = Path::new("/tmp/some/path.txt");
        let pattern = Regex::new(".*.py").unwrap();
        let patterns = vec![pattern];
        let is_match = path_is_match(path, &patterns);
        assert!(!is_match);
    }
}
