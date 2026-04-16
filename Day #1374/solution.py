# Day 1374: Jump game: greedy track furthest reachable index. Time O(n), Space O(1).


def can_reach(a):
    reach = 0
    for i, v in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + v)
    return True


if __name__ == "__main__":
    print(can_reach([2, 0, 1, 0]))
    print(can_reach([1, 1, 0, 1]))
