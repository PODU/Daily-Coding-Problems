# Day 1708: Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
def is_power_of_four(n):
    return n > 0 and (n & (n - 1)) == 0 and (n & 0x55555555) != 0


if __name__ == "__main__":
    for t in (16, 8, 1, 64, 5):
        print(f"{t} -> {str(is_power_of_four(t)).lower()}")
