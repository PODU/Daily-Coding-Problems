# Day 288: Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
# (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
def kaprekar(n):
    steps = 0
    while n != 6174:
        d = f"{n:04d}"
        asc = int("".join(sorted(d)))
        desc = int("".join(sorted(d, reverse=True)))
        n = desc - asc
        steps += 1
    return steps


if __name__ == "__main__":
    print(kaprekar(1234))
