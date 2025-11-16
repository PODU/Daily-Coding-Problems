# Day 607: Min total movement to seat M people contiguously in a row.
# Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).


def min_cost(seats):
    b = []
    idx = 0
    for i, s in enumerate(seats):
        if s == 1:
            b.append(i - idx)
            idx += 1
    if not b:
        return 0
    b.sort()
    med = b[len(b) // 2]
    return sum(abs(v - med) for v in b)


def main():
    seats = [0, 1, 1, 0, 1, 0, 0, 0, 1]
    print(min_cost(seats))  # 5


if __name__ == '__main__':
    main()
