# Day 989: Deduce coin denominations from a ways-to-make-change array.
# Walk amounts; whenever target[i] exceeds reconstructed ways, i is a coin and we fold it into the DP.
# O(N^2) time, O(N) space.


def find_denominations(target):
    n = len(target)
    have = [0] * n
    have[0] = 1                      # one way to make 0 with no coins
    coins = []
    for i in range(1, n):
        if target[i] > have[i]:      # unaccounted combinations => i is a denomination
            coins.append(i)
            for j in range(i, n):
                have[j] += have[j - i]
    return coins


if __name__ == "__main__":
    target = [1, 0, 1, 1, 2]
    coins = find_denominations(target)
    print(", ".join(map(str, coins)))  # expected: 2, 3, 4
