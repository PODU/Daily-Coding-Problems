# Day 717: Min cost to paint N houses, K colors, no two adjacent same color.
# DP over rows tracking best & second-best previous costs. Time O(N*K), space O(1).

def min_cost(costs):
    if not costs:
        return 0
    K = len(costs[0])
    prev = list(costs[0])
    for i in range(1, len(costs)):
        m1 = m2 = float("inf")
        idx = -1
        for k in range(K):
            if prev[k] < m1:
                m2, m1, idx = m1, prev[k], k
            elif prev[k] < m2:
                m2 = prev[k]
        cur = [costs[i][k] + (m2 if k == idx else m1) for k in range(K)]
        prev = cur
    return min(prev)


if __name__ == "__main__":
    print(min_cost([[1, 5, 3], [2, 9, 4]]))  # 5
