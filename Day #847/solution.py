# Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.

def can_reach(a):
    reach = 0
    for i, v in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + v)
    return True


if __name__ == "__main__":
    print(can_reach([2, 0, 1, 0]))  # True
    print(can_reach([1, 1, 0, 1]))  # False
