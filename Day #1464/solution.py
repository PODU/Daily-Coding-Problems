# Day 1464: Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
# Time: O(n), Space: O(1).
def two_unique(arr):
    x = 0
    for v in arr:
        x ^= v
    bit = x & (-x)  # lowest set bit
    a = b = 0
    for v in arr:
        if v & bit:
            a ^= v
        else:
            b ^= v
    return (a, b) if a < b else (b, a)


if __name__ == "__main__":
    arr = [2, 4, 6, 8, 10, 2, 6, 10]
    a, b = two_unique(arr)
    print(f"{a} and {b}")
