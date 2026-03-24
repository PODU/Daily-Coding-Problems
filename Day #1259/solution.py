# Day 1259: Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.

def min_flips(s):
    flips = 0
    y_count = 0
    for c in s:
        if c == 'y':
            y_count += 1
        else:
            flips = min(flips + 1, y_count)
    return flips


if __name__ == "__main__":
    print(min_flips("xyxxxyxyy"))
