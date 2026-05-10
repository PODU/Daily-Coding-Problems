# Day 1503: Power set via bitmask, sorted by (size, lexicographic) to match example order.
# Time O(n*2^n), Space O(2^n).

def power_set(s):
    n = len(s)
    subsets = []
    for mask in range(1 << n):
        sub = [s[i] for i in range(n) if mask & (1 << i)]
        subsets.append(sub)
    subsets.sort(key=lambda x: (len(x), x))
    return subsets


if __name__ == "__main__":
    subsets = power_set([1, 2, 3])
    parts = ["{" + ", ".join(map(str, sub)) + "}" for sub in subsets]
    print("{" + ", ".join(parts) + "}")
