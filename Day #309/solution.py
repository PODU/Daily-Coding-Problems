# Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
# at the median of those values. O(M log M).
def min_cost(seats):
    positions = [i for i, v in enumerate(seats) if v == 1]
    if not positions:
        return 0
    b = sorted(p - i for i, p in enumerate(positions))
    med = b[len(b) // 2]
    return sum(abs(x - med) for x in b)


if __name__ == "__main__":
    print(min_cost([0, 1, 1, 0, 1, 0, 0, 0, 1]))  # 5
