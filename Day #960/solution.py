# Day 960: jump game - can we reach the last index? Greedy furthest-reach.
# Time O(n), Space O(1).

def can_reach(a):
    reach = 0
    for i, step in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + step)
    return True


if __name__ == "__main__":
    print(str(can_reach([1, 3, 1, 2, 0, 1])).lower())  # true
    print(str(can_reach([1, 2, 1, 0, 0])).lower())     # false
