# Day 1102: Min total movement to seat people contiguously (order preserved).
# person i lands at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
# to its median. Time: O(N). Space: O(M).
def min_cost(seats):
    b = []
    p = 0
    for i, s in enumerate(seats):
        if s == 1:
            b.append(i - p)
            p += 1
    if not b:
        return 0
    b.sort()
    med = b[len(b) // 2]
    return sum(abs(x - med) for x in b)


if __name__ == "__main__":
    print(min_cost([0, 1, 1, 0, 1, 0, 0, 0, 1]))  # 5
