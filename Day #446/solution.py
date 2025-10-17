# Day 446: Power of four test in O(1) (better than O(log N)).
# Power of two (single set bit) AND that bit sits at an even position.


def is_power_of_four(n):
    return n > 0 and (n & (n - 1)) == 0 and (n & 0x55555555) != 0


if __name__ == "__main__":
    for n in (1, 4, 8, 16, 64, 0, 5, 256):
        print(f"{n} -> {is_power_of_four(n)}")
    # 1->True 4->True 8->False 16->True 64->True 0->False 5->False 256->True
