#!/usr/bin/env python3
import os
import re
import sys
from manager import manager
import shutil
# Usage: ./next.py
# Will create the next aoc problem and attempt to also pull the data for it
#
# Or...
#
# Usage: ./next.py #
# Will attempt to just grab the data for that day or, if that day already
# exists, simply open it in your browser and editor

def open_problem_description(year, day):
    os.system("firefox --new-tab https://adventofcode.com/%d/day/%d &" % (year, day))

def open_editor(path):
    os.system("cd %s && code . *" % (path))

def main(args):
    day = get_day((args + [None])[0])
    year = get_year_from_folder()

    # if we got here then it's okay, let's move on
    target = "%02d" % (day)

    # Make if doesn't already exist
    if not os.path.exists(target):
        print("getting data...")
        puzzle = manager.get_puzzle(year, day)
        print(puzzle.example_data)
        print("data got!")

        print("Making since doesn't exist...")
        shutil.copytree(os.path.join('..', 'TEMPLATE'), target)

        # write our data out
        with open(os.path.join(target, "sample.txt"), "w") as f:
            f.write(puzzle.example_data)

        with open(os.path.join(target, "input.txt"), "w") as f:
            f.write(puzzle.input_data)

    # and, finally, open our browser and editor
    open_editor(target)
    open_problem_description(year, day)


def get_day(day):
    if day: return int(day)
    return get_next_question_number()

def get_year_from_folder():
    return int(os.path.basename(os.getcwd()))

def get_next_question_number():
    return get_latest_question_number()+1

def get_latest_question_number():
    dirs = get_question_dirs()
    return max(dirs+[0])

def get_question_dirs():
    return [int(f.name) for f in os.scandir(".") if f.is_dir() and re.fullmatch("[0-9]+", f.name)]


if __name__ == "__main__":
    main(sys.argv[1:])

