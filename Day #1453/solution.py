# Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
# in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
from typing import List


def apply_permutation(arr: List[str], P: List[int]) -> None:
    for i in range(len(arr)):
        while P[i] != i:
            pi = P[i]
            arr[i], arr[pi] = arr[pi], arr[i]
            P[i], P[pi] = P[pi], P[i]


if __name__ == "__main__":
    arr = ["a", "b", "c"]
    P = [2, 1, 0]
    apply_permutation(arr, P)
    print("[" + ", ".join(f'"{x}"' for x in arr) + "]")  # ["c", "b", "a"]
