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
use regex::bytes::Regex;
use std::path::Path;

pub(crate) fn path_is_match(path: &Path, regex_patterns: &Vec<Regex>) -> bool {
    regex_patterns.len() == 0
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
        println!("{}", err);
    }
    results
}

pub fn findgrep(
    path: String,
    threads: usize,
    ignore_hidden: bool,
    buffer_size: usize,
    log_errors: bool,
    only_files: bool,
    filter_by_grep: bool,
    find_patterns: Vec<String>,
    grep_patterns: Vec<String>,
) -> Result<Vec<FindResult>> {
    let mut find_regex_patterns = Vec::new();
    for item in find_patterns {
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
    for item in grep_patterns {
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
        .hidden(ignore_hidden)
        .threads(threads)
        .build_parallel();

    let (tx, rx): (Sender<Vec<FindResult>>, Receiver<Vec<FindResult>>) = unbounded();
    walker.run(|| {
        let mut batch_sender = BatchSender::new(tx.clone(), buffer_size);
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
                            if !filter_by_grep
                                || !grep_results.is_empty()
                                || worker_grep_patterns.is_empty()
                            {
                                batch_sender.send(FindResult {
                                    path: path.to_path_buf(),
                                    path_type: "file".to_string(),
                                    grep_results: grep_results,
                                });
                            }
                        }
                    } else {
                        if !only_files {
                            if path_is_match(path, worker_find_patterns) {
                                batch_sender.send(FindResult {
                                    path: path.to_path_buf(),
                                    path_type: "directory".to_string(),
                                    grep_results: Vec::new(),
                                });
                            }
                        }
                    }
                }
                Err(err) => {
                    if log_errors {
                        println!("Error: {}", err);
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
    fn it_works() {
        assert_eq!(4, 4);
    }
}
