from .pyfindgrep import findgrep_py
from pyfindgrep.data import GrepResult, FindResult


__doc__ = pyfindgrep.__doc__
if hasattr(pyfindgrep, "__all__"):
    __all__ = pyfindgrep.__all__


def findgrep(
    path: str,
    patterns: list[str] | None = None,
    content_patterns: list[str] | None = None,
    parallel: bool | None = None,
    threads: int | None = None,
    ignore_hidden_files: bool = True,
    buffer_size: int = 1024,
    filter_by_grep: bool = True,
    log_errors: bool = False,
    only_files: bool = True,
) -> list[FindResult]:
    """
    Searches for files with names and content matching patterns.

    Similar to running find with grep.
    """
    if patterns and len(patterns) == 0:
        raise ValueError("Providing an empty list of patterns will produce no results")

    if patterns is None:
        patterns = []

    if content_patterns is None:
        content_patterns = []

    if parallel is not None and threads is not None:
        raise ValueError("Only specify one of threads or parallel")

    if threads is None:
        threads = 1

    if parallel:
        threads = 0

    raw_results: list[dict] = findgrep_py(
        path,
        threads,
        ignore_hidden_files,
        buffer_size,
        log_errors,
        only_files,
        filter_by_grep,
        patterns,
        content_patterns,
    )
    parsed_results: list[FindResult] = [
        FindResult.from_raw_result(result) for result in raw_results
    ]
    return parsed_results
