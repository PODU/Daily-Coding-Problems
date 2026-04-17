# Day 1379: Max of two without if/comparison: subtract the masked difference using the sign bit.
# max(a,b) = a - ((a-b) & ((a-b)>>63)). Time O(1), Space O(1). (values fit in 64 bits)


def max_no_branch(a, b):
    diff = a - b
    return a - (diff & (diff >> 63))


if __name__ == "__main__":
    print(max_no_branch(3, 7))    # 7
    print(max_no_branch(10, -5))  # 10
