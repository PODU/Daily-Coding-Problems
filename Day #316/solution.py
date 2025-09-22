# Day 316: Reconstruct coin denominations from a ways-to-make-change array.
# DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).

def join_coins(c):
    if not c:
        return ""
    if len(c) == 1:
        return str(c[0])
    if len(c) == 2:
        return f"{c[0]} and {c[1]}"
    return ", ".join(str(x) for x in c[:-1]) + f", and {c[-1]}"


def find_coins(A):
    n = len(A)
    ways = [0] * n
    ways[0] = 1
    coins = []
    for i in range(1, n):
        if A[i] > ways[i]:
            coins.append(i)
            for j in range(i, n):
                ways[j] += ways[j - i]
    return coins


if __name__ == "__main__":
    A = [1, 0, 1, 1, 2]
    print(join_coins(find_coins(A)))
