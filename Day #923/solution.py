# Day 923: Power of four via O(1) bit manipulation.
# power of two: n>0 && (n&(n-1))==0; bit in even position: (n & 0x55555555)!=0.
# Time O(1), Space O(1).

def is_power_of_four(n: int) -> bool:
    return n > 0 and (n & (n - 1)) == 0 and (n & 0x55555555) != 0


def main():
    for n in [1, 4, 16, 64, 8, 5, 0]:
        print(f"{n}: {str(is_power_of_four(n)).lower()}")


if __name__ == "__main__":
    main()
