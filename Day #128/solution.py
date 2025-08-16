# Day 128: Tower of Hanoi - print all moves.
# Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.


def hanoi(n, frm, via, to):
    if n == 0:
        return
    hanoi(n - 1, frm, to, via)
    print("Move %d to %d" % (frm, to))
    hanoi(n - 1, via, frm, to)


if __name__ == "__main__":
    hanoi(3, 1, 2, 3)
