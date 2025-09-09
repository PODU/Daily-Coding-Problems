# Day 240: Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
# N - (number of connected components among couples). Time: O(N alpha), Space: O(N).


def min_swaps(row):
    n = len(row) // 2
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    comps = n
    for i in range(0, len(row), 2):
        a, b = find(row[i] // 2), find(row[i + 1] // 2)
        if a != b:
            parent[a] = b
            comps -= 1
    return n - comps


if __name__ == "__main__":
    print(min_swaps([0, 2, 1, 3]))  # 1
