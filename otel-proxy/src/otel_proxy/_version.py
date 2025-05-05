from pathlib import Path

import tomllib


def get_version():
    pyproject = Path(__file__).parents[2] / "pyproject.toml"
    with pyproject.open("rb") as f:
        data = tomllib.load(f)
    return data["project"]["version"]
