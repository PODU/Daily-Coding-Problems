# Day 19: Paint House: DP tracking min cost per color using min1/min2 trick.
# Time O(N*K), Space O(1) extra.
def min_cost(cost):
    if not cost:
        return 0
    K = len(cost[0])
    prev = list(cost[0])
    for i in range(1, len(cost)):
        min1 = min2 = -1
        for k in range(K):
            if min1 == -1 or prev[k] < prev[min1]:
                min2, min1 = min1, k
            elif min2 == -1 or prev[k] < prev[min2]:
                min2 = k
        cur = [0] * K
        for k in range(K):
            best = prev[min2] if k == min1 else prev[min1]
            cur[k] = cost[i][k] + best
        prev = cur
    return min(prev)


if __name__ == "__main__":
    cost = [[1, 5, 3], [2, 9, 4]]
    print("Minimum cost =", min_cost(cost))
