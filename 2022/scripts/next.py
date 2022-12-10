#!/usr/bin/env python3
import os
import re
import sys
from manager import manager
import datetime
import shutil
# Usage: ./next.py
# Will create the next aoc problem and attempt to also pull the data for it
# Or...
# Usage: ./next.py #
# Will attempt to just

def main(args):
    next_day = get_next_question_number()
    year = get_current_year()
    print("getting data...")
    puzzle = manager.get_puzzle(year, next_day)
    print(puzzle.example_data)
    print("data got!")

    # if we got here then it's okay, let's move on
    target = "%02d" % (next_day)
    shutil.copytree(os.path.join('..', 'TEMPLATE'), target)

    # write our data out
    with open(os.path.join(target, "sample.txt"), "w") as f:
        f.write(puzzle.example_data)

    with open(os.path.join(target, "input.txt"), "w") as f:
        f.write(puzzle.input_data)

    # and, finally, open our browser and editor
    os.system("firefox --new-tab https://adventofcode.com/%d/day/%d &" % (year, next_day))
    os.system("cd %s && code . *" % (target))


def get_current_year():
    today = datetime.date.today()
    return today.year


def get_next_question_number():
    return get_latest_question_number()+1

def get_latest_question_number():
    dirs = get_question_dirs()
    return max(dirs+[0])

def get_question_dirs():
    return [int(f.name) for f in os.scandir(".") if f.is_dir() and re.fullmatch("[0-9]+", f.name)]


if __name__ == "__main__":
    main(sys.argv[1:])

