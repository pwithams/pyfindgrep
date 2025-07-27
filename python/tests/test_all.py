import tempfile
import os
import pyfindgrep


DEFAULT_FILENAMES = [
    "a.foo",
    "one/b.foo",
    "one/two/c.foo",
    "one/two/C.Foo2",
    "one/two/three/d.foo",
    "foo.foo",
    "foo.bar",
    ".hidden.txt",
]


def create_working_directory(directory, filenames, file_content="foo"):
    created = {
        "default": [],
        "hidden": [],
        "content": [],
    }
    for index, filename in enumerate(filenames):
        full_name = os.path.join(directory, filename)
        os.makedirs(os.path.dirname(full_name), exist_ok=True)
        content = file_content if index % 2 == 0 else ""
        with open(full_name, "w") as f:
            f.write(content)

        entry = {"path": full_name, "content": content}

        if filename.startswith("."):
            created["hidden"].append(entry)
        else:
            created["default"].append(entry)

        if content:
            created["content"].append(entry)

    return created


def test_pyfindgrep():
    filenames = DEFAULT_FILENAMES
    with tempfile.TemporaryDirectory() as tmpdirname:
        created = create_working_directory(tmpdirname, filenames)

        results = pyfindgrep.findgrep(tmpdirname)
    result_filenames = [findresult.path for findresult in results]
    expected_filenames = [
        os.path.join(tmpdirname, filename["path"]) for filename in created["default"]
    ]
    assert result_filenames == expected_filenames


def test_pyfindgrep_hidden():
    filenames = DEFAULT_FILENAMES
    with tempfile.TemporaryDirectory() as tmpdirname:
        created = create_working_directory(tmpdirname, filenames)

        results = pyfindgrep.findgrep(tmpdirname, ignore_hidden_files=False)
    result_filenames = [findresult.path for findresult in results]
    hidden_filenames = [
        os.path.join(tmpdirname, filename["path"]) for filename in created["hidden"]
    ]
    for filename in hidden_filenames:
        assert filename in result_filenames


def test_pyfindgrep_find_pattern():
    filenames = DEFAULT_FILENAMES
    with tempfile.TemporaryDirectory() as tmpdirname:
        created = create_working_directory(tmpdirname, filenames)

        results = pyfindgrep.findgrep(tmpdirname, find_patterns=[".*bar$"])
    result_filenames = [os.path.basename(findresult.path) for findresult in results]
    expected_filenames = ["foo.bar"]
    assert result_filenames == expected_filenames


def test_pyfindgrep_grep_pattern():
    filenames = DEFAULT_FILENAMES
    with tempfile.TemporaryDirectory() as tmpdirname:
        created = create_working_directory(tmpdirname, filenames, file_content="foo")

        results = pyfindgrep.findgrep(tmpdirname, grep_patterns=["foo"])
    result_filenames = [findresult.path for findresult in results]
    expected_filenames = [
        os.path.join(tmpdirname, filename["path"]) for filename in created["content"]
    ]
    assert result_filenames == expected_filenames
