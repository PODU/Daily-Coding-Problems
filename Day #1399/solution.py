# Day 1399: DP over rows, tracking the two smallest running totals so each house picks
# the best previous color != its own. Time O(N*K), Space O(1) extra.

def min_cost(costs):
    if not costs:
        return 0
    K = len(costs[0])
    prev = list(costs[0])
    for i in range(1, len(costs)):
        m1 = m2 = float('inf')
        idx1 = -1
        for k in range(K):
            if prev[k] < m1:
                m2, m1, idx1 = m1, prev[k], k
            elif prev[k] < m2:
                m2 = prev[k]
        prev = [costs[i][k] + (m2 if k == idx1 else m1) for k in range(K)]
    return min(prev)


if __name__ == "__main__":
    print(min_cost([[1, 5, 3], [2, 9, 4]]))  # 5
