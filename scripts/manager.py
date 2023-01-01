#!/usr/bin/env python3
import os
from aocd import get_data, models
from dotenv import load_dotenv
import shutil

load_dotenv()

class manager():
    def get_data(year, day):
        return get_data(day=day, year=year)

    def get_puzzle(year, day):
        user = models.default_user()
        puzzle = models.Puzzle(year=year, day=day, user=user)
        return puzzle

    def open_day(year, day):
        target = manager.get_target(day)
        manager.open_editor(target)
        manager.open_problem_description(year, day)

    def open_problem_description(year, day):
        os.system("firefox --new-tab https://adventofcode.com/%d/day/%d &" % (year, day))

    def open_editor(path):
        os.system("cd %s && vim . &" % (path))

    def download_day(year, day, do_open=False):
        target = manager.get_target(day)

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
        if do_open: manager.open_day(year, day)

    def get_target(day):
        return "%02d" % (day)


def main():
    pass

if __name__ == "__main__":
    main()

