# Day 1480: Sort a million ints in [0, 1e9] without a billion-element array.
# Key insight: never allocate an array indexed by value (that would be ~1e9).
# Index by element count instead. LSD radix sort (base 256) is O(d*N) ~ O(N),
# touching only N elements plus a 256-bucket table. For truly out-of-core data,
# the same idea generalizes to an external merge sort: sort chunks that fit in
# RAM, spill to disk, then k-way merge.
# Time O(N) (4 passes for 32-bit), Space O(N).

def radix_sort(arr):
    if not arr:
        return arr
    out = list(arr)
    tmp = [0] * len(out)
    for shift in (0, 8, 16, 24):  # base-256 digits of a 32-bit key
        count = [0] * 257
        for v in out:
            count[((v >> shift) & 0xFF) + 1] += 1
        for i in range(256):
            count[i + 1] += count[i]
        for v in out:
            d = (v >> shift) & 0xFF
            tmp[count[d]] = v
            count[d] += 1
        out, tmp = tmp, out
    return out


if __name__ == "__main__":
    print(radix_sort([9, 11, 8, 5, 7, 10]))  # [5, 7, 8, 9, 10, 11]
