# Day 1827: Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
# O(eggs * answer). For N=1,k=5 -> 5.
def floors_covered(t, eggs, cap):
    total = 0
    c = 1
    for i in range(1, eggs + 1):
        c = c * (t - i + 1) // i   # C(t, i) incrementally
        total += c
        if total >= cap:
            return cap
    return total


def min_drops(eggs, k):
    t = 0
    while floors_covered(t, eggs, k) < k:
        t += 1
    return t


if __name__ == "__main__":
    print(min_drops(1, 5))  # 5
