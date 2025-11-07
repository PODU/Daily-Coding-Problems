# Day 565: Jump Game: greedy tracking farthest reachable index. Time O(N), Space O(1).
def can_jump(nums):
    farthest = 0
    for i, v in enumerate(nums):
        if i > farthest:
            return False
        farthest = max(farthest, i + v)
    return True


if __name__ == "__main__":
    print("True" if can_jump([2, 0, 1, 0]) else "False")
    print("True" if can_jump([1, 1, 0, 1]) else "False")
