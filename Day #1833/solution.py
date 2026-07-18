# Day 1833: Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
# '+' takes the current low, '-' takes the current high; produces a valid order.
def reconstruct(signs):  # signs: list of '+'/'-' (the leading None is dropped)
    L = len(signs) + 1
    N = L - 1
    res = [0] * L
    low, high = 0, N
    for j, s in enumerate(signs):
        if s == "+":
            res[j] = low
            low += 1
        else:
            res[j] = high
            high -= 1
    res[L - 1] = low
    return res


if __name__ == "__main__":
    # input [None, +, +, -, +] -> constraints (+, +, -, +)
    signs = ["+", "+", "-", "+"]
    print(reconstruct(signs))  # a valid reconstruction, e.g. [0, 1, 4, 2, 3]
