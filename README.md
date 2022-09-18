# YAPB: A Minimalist Progress Bar for Python

[example workflow](https://github.com/github/docs/actions/workflows/rust_tests.yml/badge.svg)

## Usage

```python
from yapb import ProgressBar

N = 100
pb = ProgressBar(N, message="Computing...", description="Progress")
for _ in range(N):
    pb.update(message="Still computing...", description=f"At {_}/{N}")
    pb.render()
```

## Installation

```pip install yapb```
