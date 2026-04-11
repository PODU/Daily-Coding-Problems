# Day 1339: Bishops attack along diagonals: group by (row-col) and (row+col); each group of k adds k*(k-1)/2. O(N) time, O(N) space.
from collections import Counter


def count_attacking_pairs(m, bishops):
    diag = Counter(r - c for r, c in bishops)
    anti = Counter(r + c for r, c in bishops)
    pairs = sum(k * (k - 1) // 2 for k in diag.values())
    pairs += sum(k * (k - 1) // 2 for k in anti.values())
    return pairs


def main():
    m = 5
    bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]
    print(count_attacking_pairs(m, bishops))


if __name__ == "__main__":
    main()
