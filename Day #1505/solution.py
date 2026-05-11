# Day 1505: Jump game: greedy, track furthest reachable index.
# Time O(n), Space O(1).

def can_jump(a):
    reach = 0
    for i, v in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + v)
    return True


if __name__ == "__main__":
    print(can_jump([2, 0, 1, 0]))
