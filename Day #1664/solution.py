# Day 1664: Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
def main():
    arr = ["a", "b", "c"]
    P = [2, 1, 0]
    res = [None] * len(arr)
    for i in range(len(arr)):
        res[P[i]] = arr[i]
    print("[" + ", ".join(res) + "]")

if __name__ == "__main__":
    main()
