# Day 401: Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
# (In-place alternative: follow cycles swapping elements.)
def apply_permutation(array, P):
    result = [None] * len(array)
    for i, val in enumerate(array):
        result[P[i]] = val
    return result


def main():
    array = ["a", "b", "c"]
    P = [2, 1, 0]
    res = apply_permutation(array, P)
    print("[" + ", ".join('"' + x + '"' for x in res) + "]")


if __name__ == "__main__":
    main()
