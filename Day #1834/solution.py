# Day 1834: Min swaps to seat couples together. Union couples in each seat-pair; answer is
# N - (#connected components). O(N * alpha(N)).
def min_swaps(row):
    n = len(row) // 2  # number of couples
    par = list(range(n))

    def find(x):
        while par[x] != x:
            par[x] = par[par[x]]
            x = par[x]
        return x

    comps = n
    for i in range(n):
        ra, rb = find(row[2 * i] // 2), find(row[2 * i + 1] // 2)
        if ra != rb:
            par[ra] = rb
            comps -= 1
    return n - comps


if __name__ == "__main__":
    row = [0, 2, 1, 3]  # couples are (0,1) and (2,3)
    print(min_swaps(row))  # 1
