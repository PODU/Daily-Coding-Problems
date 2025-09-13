# Day 268: Power of four check in O(1).
# Power of two (n & (n-1))==0 AND single bit at even position (n & 0x55555555). Time O(1), Space O(1).

def is_power_of_four(n: int) -> bool:
    return n != 0 and (n & (n - 1)) == 0 and (n & 0x55555555) != 0


if __name__ == "__main__":
    for t in (16, 8, 64, 5):
        print(f"{t} -> {is_power_of_four(t)}")
