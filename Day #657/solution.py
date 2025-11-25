# Day 657: Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
def power_set(s):
    n = len(s)
    subsets = []
    for mask in range(1 << n):
        sub = [s[i] for i in range(n) if mask & (1 << i)]
        subsets.append(sub)
    subsets.sort(key=lambda x: (len(x), x))
    return subsets

if __name__ == "__main__":
    s = [1, 2, 3]
    subsets = power_set(s)
    parts = ["{" + ", ".join(str(x) for x in sub) + "}" for sub in subsets]
    print("{" + ", ".join(parts) + "}")
