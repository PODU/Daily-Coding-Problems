# Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
# digit arrangement until 6174; count steps. Time O(steps), Space O(1).


def kaprekar_steps(n):
    steps = 0
    while n != 6174:
        s = f"{n:04d}"
        asc = int("".join(sorted(s)))
        desc = int("".join(sorted(s, reverse=True)))
        n = desc - asc
        steps += 1
    return steps


if __name__ == "__main__":
    print(kaprekar_steps(1234))
