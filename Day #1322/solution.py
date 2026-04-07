# Day 1322: Misere Nim (last stone loses) forced win for first player.
# Theorem: first player wins iff (some heap>1 and XOR!=0) OR (all heaps<=1 and XOR==0). O(n) time, O(1) space.
from functools import reduce


def first_player_wins(heaps):
    x = reduce(lambda a, b: a ^ b, heaps, 0)
    if max(heaps) <= 1:
        return x == 0
    return x != 0


if __name__ == "__main__":
    print(first_player_wins([3, 4, 5]))  # True
