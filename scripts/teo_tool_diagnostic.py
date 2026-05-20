"""Quick diagnostic for TEO local project-analysis tools.

Checks the Rust extension and the OpenJarvis tools that let TEO inspect
local projects by path.
"""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "target",
        nargs="?",
        default=".",
        help="Project directory or file to inspect.",
    )
    args = parser.parse_args()

    target = Path(args.target).resolve()

    print(f"Target: {target}")

    try:
        import openjarvis_rust

        print(f"openjarvis_rust: OK ({openjarvis_rust.__name__})")
    except Exception as exc:
        print(f"openjarvis_rust: FAIL ({exc})")
        return 1

    from openjarvis.tools.file_read import FileReadTool
    from openjarvis.tools.shell_exec import ShellExecTool

    shell = ShellExecTool()
    cmd = "dir /b" if target.is_dir() else "cd"
    shell_result = shell.execute(
        command=cmd,
        working_dir=str(target if target.is_dir() else target.parent),
    )
    print(f"shell_exec: {'OK' if shell_result.success else 'FAIL'}")
    print(shell_result.content[:1000])

    if target.is_file():
        sample_file = target
    else:
        candidates = [
            target / "README.md",
            target / "pyproject.toml",
            target / "package.json",
        ]
        sample_file = next((path for path in candidates if path.exists()), None)

    if sample_file is None:
        print("file_read: SKIP (no README.md, pyproject.toml, or package.json found)")
        return 0 if shell_result.success else 1

    read_result = FileReadTool().execute(path=str(sample_file), max_lines=20)
    print(f"file_read: {'OK' if read_result.success else 'FAIL'} ({sample_file})")
    print(read_result.content[:1000])

    return 0 if shell_result.success and read_result.success else 1


if __name__ == "__main__":
    raise SystemExit(main())
