# Day 106: Jump Game - greedy max-reach. O(n) time, O(1) space.
def can_reach(a):
    reach = 0
    for i, x in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + x)
    return True


if __name__ == "__main__":
    print(can_reach([2, 0, 1, 0]))
    print(can_reach([1, 1, 0, 1]))
