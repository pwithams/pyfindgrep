use grep_regex::RegexMatcher;
use std::path::PathBuf;

pub(crate) struct RegexMatcherInfo {
    pub(crate) matcher: RegexMatcher,
    pub(crate) pattern: String,
}

#[derive(Debug, Clone)]
pub struct GrepResult {
    pub pattern: String,
    pub matching_text: String,
    pub matching_line: String,
    pub lineno: u64,
}

#[derive(Debug, Clone)]
pub struct FindResult {
    pub path: PathBuf,
    pub path_type: String,
    pub grep_results: Vec<GrepResult>,
}
