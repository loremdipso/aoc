# Michael's Advent of Code

This repo holds all my AOC code, including some handy config for quick
setup.

# Setup

Put your base program in the `TEMPLATE` folder. After setup it'll have its
`input.txt` and `sample.txt` files filled with data from AOC.

For convenience this repo uses a python package to fetch the data via
its API: https://github.com/wimglenn/advent-of-code-data. To use,
install with:

```
pip install advent-of-code-data
```

Then you need to set AOC_SESSION (grabbed from your browser's cookies)
in a .env file in the root directory:

```
export AOC_SESSION=xxx
```

# Usage

Create a folder named after the AOC year. From that folder call:

```
../scripts/next.py
```

To get the next AOC day. Or to get a specific day:

```
../scripts/next.py [DAY]
```

Note this command might fail for any number of reasons, including if it
can't connect, if your browser cookie is missing or expired, or if the
year+day combination is incorrect. If it fails a new folder will not be
created.
