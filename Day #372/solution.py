# Day 372: Count digits of a natural number without loops.
# Recursion: 1 digit for n<10, else 1 + digits(n//10). Time O(d), Space O(d).


def num_digits(n):
    return 1 if n < 10 else 1 + num_digits(n // 10)


if __name__ == "__main__":
    print(num_digits(0))      # 1
    print(num_digits(7))      # 1
    print(num_digits(42))     # 2
    print(num_digits(12345))  # 5
