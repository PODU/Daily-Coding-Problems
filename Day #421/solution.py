# Day 421: LSD radix sort (base-256). O(n*w) time, O(n) space.
# Sorts 32-bit ints in 4 byte-passes without allocating a billion-element array.
def radix_sort(a):
    a = list(a)
    for shift in range(0, 32, 8):
        cnt = [0] * 257
        for x in a:
            cnt[((x >> shift) & 0xFF) + 1] += 1
        for i in range(256):
            cnt[i + 1] += cnt[i]
        buf = [0] * len(a)
        for x in a:
            d = (x >> shift) & 0xFF
            buf[cnt[d]] = x
            cnt[d] += 1
        a = buf
    return a


if __name__ == "__main__":
    a = [5, 3, 1000000000, 0, 42, 7, 42]
    print("Sorted:", radix_sort(a))
