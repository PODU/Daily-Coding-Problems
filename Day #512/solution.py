# Day 512: Jump Game: greedily track the furthest reachable index. O(n) time, O(1) space.
def can_jump(a):
    reach = 0
    for i, v in enumerate(a):
        if i > reach:
            return False
        reach = max(reach, i + v)
    return True


if __name__ == "__main__":
    print(str(can_jump([1, 3, 1, 2, 0, 1])).lower())
    print(str(can_jump([1, 2, 1, 0, 0])).lower())
