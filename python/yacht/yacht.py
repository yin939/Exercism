"""
This exercise stub and the test suite contain several enumerated constants.

Since Python 2 does not have the enum module, the idiomatic way to write
enumerated constants has traditionally been a NAME assigned to an arbitrary,
but unique value. An integer is traditionally used because it’s memory
efficient.
It is a common practice to export both constants and functions that work with
those constants (ex. the constants in the os, subprocess and re modules).

You can learn more here: https://en.wikipedia.org/wiki/Enumerated_type
"""


from collections import Counter
# Score categories.
# Change the values as you see fit.
from typing import List

YACHT = 0
ONES = 1
TWOS = 2
THREES = 3
FOURS = 4
FIVES = 5
SIXES = 6
FULL_HOUSE = 7
FOUR_OF_A_KIND = 8
LITTLE_STRAIGHT = 9
BIG_STRAIGHT = 10
CHOICE = 11


def score(dice: List, category):
    if len(dice) !=5:
        return 0

    counter = Counter(dice)
    if category in [ONES, TWOS, THREES, FOURS, FIVES, SIXES]:
        return counter[category] * category

    if category == FULL_HOUSE:
        arr = counter.most_common(2)
        if arr[0][1] == 3 and arr[1][1] == 2:
            return sum(dice)

        return 0

    if category == FOUR_OF_A_KIND:
        arr = counter.most_common(1)[0]
        if arr[1] >= 4:
            return arr[0] * 4

        return 0

    if category == LITTLE_STRAIGHT:
        if sorted(dice) == [1, 2, 3, 4, 5]:
            return 30

        return 0

    if category == BIG_STRAIGHT:
        if sorted(dice) == [2, 3, 4, 5, 6]:
            return 30

        return 0

    if category == CHOICE:
        return sum(dice)

    if category == YACHT:
        if len(set(dice)) == 1:
            return 50

        return 0

    return 0
