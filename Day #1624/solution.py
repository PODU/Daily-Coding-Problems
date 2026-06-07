# Day 1624: Steps of Kaprekar's routine to reach 6174.
# Iterate sort-desc minus sort-asc until 6174. Time O(1) (bounded ~7 iters).
def kaprekar_steps(n):
    steps = 0
    while n != 6174:
        s = f"{n:04d}"
        asc = int("".join(sorted(s)))
        desc = int("".join(sorted(s, reverse=True)))
        n = desc - asc
        steps += 1
        if n == 0:
            break
    return steps


if __name__ == "__main__":
    print(kaprekar_steps(1234))
