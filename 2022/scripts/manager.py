#!/usr/bin/env python3
from aocd import get_data, models
from dotenv import load_dotenv

load_dotenv()

class manager():
    def get_data(year, day):
        return get_data(day=day, year=year)

    def get_puzzle(year, day):
        user = models.default_user()
        puzzle = models.Puzzle(year=year, day=day, user=user)
        return puzzle

def main():
    pass

if __name__ == "__main__":
    main()

