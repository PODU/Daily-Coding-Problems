# Day 289: Misere Nim (3 heaps): first player wins iff
# (some heap>1 and xor!=0) or (all heaps<=1 and xor==0). Time: O(1), Space: O(1).
def first_player_wins(heaps):
    a, b, c = heaps
    x = a ^ b ^ c
    any_big = a > 1 or b > 1 or c > 1
    return x != 0 if any_big else x == 0


if __name__ == "__main__":
    print(str(first_player_wins([3, 4, 5])).lower())
