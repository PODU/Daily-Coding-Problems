# Day 1298: Count Kaprekar steps to reach 6174.
# Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.


def kaprekar_steps(n: int) -> int:
    steps = 0
    while n != 6174:
        s = f"{n:04d}"
        asc = int("".join(sorted(s)))
        desc = int("".join(sorted(s, reverse=True)))
        n = desc - asc
        steps += 1
        if n == 0:  # all digits equal
            break
    return steps


if __name__ == "__main__":
    print(kaprekar_steps(1234))  # 3
