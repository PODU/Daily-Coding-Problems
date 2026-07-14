# Day 1806: Tower of Hanoi: recursive divide-and-conquer.
# Time: O(2^n) moves (optimal/minimal). Space: O(n) recursion depth.


def hanoi(n, frm, to, via):
    if n == 0:
        return
    hanoi(n - 1, frm, via, to)
    print(f"Move {frm} to {to}")
    hanoi(n - 1, via, to, frm)


if __name__ == "__main__":
    hanoi(3, 1, 3, 2)
