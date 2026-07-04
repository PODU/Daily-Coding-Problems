# Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
# (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
# 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
# If even the million don't fit, fall back to external merge sort (chunk-sort to
# disk, then k-way merge).
def radix_sort(a):
    a = list(a)
    tmp = [0] * len(a)
    shift = 0
    while shift < 32:
        count = [0] * 256
        for x in a:
            count[(x >> shift) & 0xFF] += 1
        total = 0
        for i in range(256):
            count[i], total = total, total + count[i]
        for x in a:
            d = (x >> shift) & 0xFF
            tmp[count[d]] = x
            count[d] += 1
        a, tmp = tmp, a
        shift += 8
    return a


if __name__ == "__main__":
    data = [999999999, 0, 123456789, 42, 1000000000, 7, 500000000]
    print(radix_sort(data))
