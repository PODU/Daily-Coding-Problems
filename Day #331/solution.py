# Day 331: Min flips so all x's precede all y's. Greedy: res=min(res+1, yCount).
# Time O(n), Space O(1).
def min_flips(s):
    res = 0
    y_count = 0
    for ch in s:
        if ch == 'y':
            y_count += 1
        else:
            res = min(res + 1, y_count)
    return res


if __name__ == "__main__":
    print(min_flips("xyxxxyxyy"))
