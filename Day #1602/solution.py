# Day 1602: Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
# flipping current 'y'->'x' costs all prior y's; flipping current 'x'->'y' costs flips+1. Time O(n), space O(1).


def min_flips(s):
    flips = 0
    y = 0
    for c in s:
        if c == 'y':
            y += 1
        else:
            flips = min(y, flips + 1)
    return flips


def main():
    print(min_flips("xyxxxyxyy"))  # 2


if __name__ == "__main__":
    main()
