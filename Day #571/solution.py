# Day 571: Tower of Hanoi: classic recursion. Move n disks from rod `frm` to `to` using `via`.
# Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.

def hanoi(n, frm, to, via):
    if n == 0:
        return
    hanoi(n - 1, frm, via, to)
    print(f"Move {frm} to {to}")
    hanoi(n - 1, via, to, frm)


if __name__ == "__main__":
    hanoi(3, 1, 3, 2)
