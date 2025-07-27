# Day 37: Power set via bitmask 0..2^n-1, then order subsets by (size, element order).
# O(2^n * n) time, O(2^n * n) space.
def power_set(s):
    n = len(s)
    subsets = []
    for mask in range(1 << n):
        sub = [s[i] for i in range(n) if mask & (1 << i)]
        subsets.append(sub)
    subsets.sort(key=lambda sub: (len(sub), sub))
    return subsets


if __name__ == "__main__":
    subsets = power_set([1, 2, 3])
    parts = ["{" + ", ".join(str(x) for x in sub) + "}" for sub in subsets]
    print("{" + ", ".join(parts) + "}")
