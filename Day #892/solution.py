# Day 892: Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
# Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.

def power_set(s):
    n = len(s)
    subsets = []
    for mask in range(1 << n):
        cur = [s[i] for i in range(n) if mask & (1 << i)]
        subsets.append(cur)
    subsets.sort(key=lambda x: (len(x), x))
    return subsets


def main():
    s = [1, 2, 3]
    subsets = power_set(s)
    parts = ["{" + ", ".join(str(x) for x in sub) + "}" for sub in subsets]
    print("{" + ", ".join(parts) + "}")


if __name__ == "__main__":
    main()
