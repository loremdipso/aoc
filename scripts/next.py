#!/usr/bin/env python3
import os
import re
import sys
from manager import manager
# Usage: ./next.py
# Will create the next aoc problem and attempt to also pull the data for it
#
# Or...
#
# Usage: ./next.py #
# Will attempt to just grab the data for that day or, if that day already
# exists, simply open it in your browser and editor

def main(args):
    # Ensure at least one argument
    args = args + [None]

    year = get_year_from_folder()

    if args[0] == "all":
        while True:
            day = get_next_question_number()
            manager.download_day(year, day, False)
    else:
        day = get_day((args + [None])[0])
        manager.download_day(year, day, True)

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

