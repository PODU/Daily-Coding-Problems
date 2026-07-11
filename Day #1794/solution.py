# Day 1794: Misere Nim: P-position (loss for mover) iff (all heaps==1 with even count) or (some heap>1 and xor==0). First wins otherwise.
# Time O(n), Space O(1).
from functools import reduce

def first_player_wins(heaps):
    x = reduce(lambda a, b: a ^ b, heaps, 0)
    if max(heaps) <= 1:
        p_position = (x == 0)   # all heaps == 1: P iff even count
    else:
        p_position = (x == 0)   # some heap > 1: P iff xor == 0
    return not p_position

if __name__ == "__main__":
    heaps = [3, 4, 5]
    print(str(first_player_wins(heaps)).lower())  # expected true
