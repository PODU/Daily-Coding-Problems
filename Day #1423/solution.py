# Day 1423: can you reach the end of the array (each value = max steps forward)?
# Approach: greedy, track farthest reachable index. O(n) time, O(1) space.


def can_reach_end(nums):
    farthest = 0
    for i, step in enumerate(nums):
        if i > farthest:
            return False
        farthest = max(farthest, i + step)
    return True


if __name__ == "__main__":
    print("true" if can_reach_end([1, 3, 1, 2, 0, 1]) else "false")  # true
    print("true" if can_reach_end([1, 2, 1, 0, 0]) else "false")     # false
