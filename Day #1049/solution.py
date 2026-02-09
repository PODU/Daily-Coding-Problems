# Day 1049: LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
# Avoids a 1e9-size counting array; memory scales with N, not value range. Else: external merge sort.

def radix_sort(a):
    buf = [0] * len(a)
    for shift in range(0, 32, 8):
        count = [0] * 256
        for v in a:
            count[(v >> shift) & 0xFF] += 1
        s = 0
        for i in range(256):
            c = count[i]; count[i] = s; s += c
        for v in a:
            d = (v >> shift) & 0xFF
            buf[count[d]] = v
            count[d] += 1
        a, buf = buf, a
    return a


if __name__ == "__main__":
    a = [829, 3, 1000000000, 42, 17, 999, 256, 0, 524287, 42]
    a = radix_sort(a)
    print(" ".join(map(str, a)))
