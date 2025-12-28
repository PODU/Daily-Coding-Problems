# Day 816: Branchless select via bitwise mask: y ^ ((-b) & (x ^ y)). O(1) time, O(1) space.

def select(x, y, b):
    return y ^ ((-b) & (x ^ y))


if __name__ == "__main__":
    print("b=1 ->", select(5, 9, 1))
    print("b=0 ->", select(5, 9, 0))
