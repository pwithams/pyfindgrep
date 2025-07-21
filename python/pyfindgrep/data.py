from dataclasses import dataclass


@dataclass
class GrepResult:
    pattern: str
    matching_text: str
    matching_line: str
    lineno: int

    @classmethod
    def from_raw_result(cls, raw_result: dict):
        return cls(
            pattern=raw_result["pattern"],
            matching_text=raw_result["matching_text"],
            matching_line=raw_result["matching_line"],
            lineno=raw_result["lineno"],
        )


@dataclass
class FindResult:
    path: str
    path_type: str
    grep_results: list[GrepResult]

    @classmethod
    def from_raw_result(cls, raw_result: dict):
        return cls(
            path=raw_result["path"],
            path_type=raw_result["path_type"],
            grep_results=[
                GrepResult.from_raw_result(result)
                for result in raw_result["grep_results"]
            ],
        )
