from pyfindgrep.pyfindgrep_lib import findgrep_py
from pyfindgrep.data import GrepResult, FindResult


def findgrep(
    path: str,
    find_patterns: list[str] | None = None,
    grep_patterns: list[str] | None = None,
    parallel: bool | None = None,
    threads: int | None = None,
    ignore_hidden_files: bool = True,
    buffer_size: int = 1024,
    log_errors: bool = False,
    only_files: bool = True,
) -> list[FindResult]:
    """
    Searches for files with names and content matching patterns.

    Similar to running find with grep.

    An find empty pattern list will match all files, and an empty grep pattern list
    will match all file contents.
    """
    # match all files if no patterns are specified
    if find_patterns is None:
        find_patterns = []

    # match all file contents if no patterns are specified
    if grep_patterns is None:
        grep_patterns = []

    if parallel is not None and threads is not None:
        raise ValueError("Only specify one of threads or parallel")

    # default to single thread if no threads are specified
    if threads is None:
        threads = 1

    # auto-detect threads if parallel is specified
    if parallel:
        threads = 0

    raw_results: list[dict] = findgrep_py(
        path,
        threads,
        ignore_hidden_files,
        buffer_size,
        log_errors,
        only_files,
        find_patterns,
        grep_patterns,
    )
    parsed_results: list[FindResult] = [
        FindResult.from_raw_result(result) for result in raw_results
    ]
    return parsed_results
