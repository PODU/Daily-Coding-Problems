# Day 595: Kaprekar's routine: repeatedly subtract ascending-digit number from
# descending-digit number (4-digit, zero-padded) until reaching 6174.
# Bounded to <=7 steps. Time O(1), Space O(1).

def main():
    n = 1234
    steps = 0
    while n != 6174:
        s = f"{n:04d}"
        asc = int("".join(sorted(s)))
        desc = int("".join(sorted(s, reverse=True)))
        n = desc - asc
        steps += 1
        print(f"{desc:04d} - {asc:04d} = {n:04d}")
    print(f"Steps: {steps}")


if __name__ == "__main__":
    main()
