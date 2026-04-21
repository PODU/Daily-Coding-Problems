# Day 1400: Power of four iff: positive, single set bit (n & (n-1))==0, and that bit sits
# at an even position (n & 0x55555555). O(1) time, O(1) space.

def is_power_of_four(n):
    return n > 0 and (n & (n - 1)) == 0 and (n & 0x55555555) != 0


if __name__ == "__main__":
    for n in [1, 4, 16, 5, 64, 63]:
        print(n, "->", str(is_power_of_four(n)).lower())
