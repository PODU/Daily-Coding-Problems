# Day 1141: Min cost to pack people (remove gaps).
# Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
def min_cost(seats):
    q = [i - idx for idx, i in enumerate(p for p, v in enumerate(seats) if v)]
    if not q:
        return 0
    q.sort()
    med = q[len(q) // 2]
    return sum(abs(v - med) for v in q)


if __name__ == "__main__":
    print(min_cost([0, 1, 1, 0, 1, 0, 0, 0, 1]))  # 5
