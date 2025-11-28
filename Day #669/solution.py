# Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
# (all heaps<=1 and xor==0). Time O(n), Space O(1).
from functools import reduce


def first_player_wins(heaps):
    x = reduce(lambda a, b: a ^ b, heaps, 0)
    any_big = any(h > 1 for h in heaps)
    return x != 0 if any_big else x == 0


if __name__ == "__main__":
    print(first_player_wins([3, 4, 5]))  # True
