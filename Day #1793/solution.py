# Day 1793: Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
# Time O(n), Space O(m).
def min_cost(seats):
    q = [j - i for i, j in enumerate(p for p, s in enumerate(seats) if s == 1)]
    if not q:
        return 0
    med = q[len(q) // 2]
    return sum(abs(v - med) for v in q)

if __name__ == "__main__":
    seats = [0, 1, 1, 0, 1, 0, 0, 0, 1]
    print(min_cost(seats))  # expected 5
