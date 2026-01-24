# Day 948: Tower of Hanoi - print all moves to move n disks from rod 1 to rod 3.
# Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.

def hanoi(n, frm, to, aux):
    if n == 0:
        return
    hanoi(n - 1, frm, aux, to)
    print(f"Move {frm} to {to}")
    hanoi(n - 1, aux, to, frm)


if __name__ == "__main__":
    hanoi(3, 1, 3, 2)
