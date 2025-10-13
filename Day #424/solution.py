# Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
# XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
def two_unique(a):
    x = 0
    for v in a:
        x ^= v
    bit = x & (-x)
    p = q = 0
    for v in a:
        if v & bit:
            p ^= v
        else:
            q ^= v
    return min(p, q), max(p, q)


if __name__ == "__main__":
    lo, hi = two_unique([2, 4, 6, 8, 10, 2, 6, 10])
    print(f"{lo} and {hi}")
