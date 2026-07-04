// Day 1763: Sort ~1e6 ints in range [0,1e9]. The MILLION elements fit in memory
// (~4MB); only the billion VALUE range is large. Use LSD radix sort (base 256,
// 4 passes) — O(n) time, O(n) space, optimal for fixed-width integers.
// If even the million don't fit, fall back to external merge sort (chunk-sort to
// disk, then k-way merge).
import java.util.*;

public class Solution {
    static void radixSort(long[] a) {
        long[] tmp = new long[a.length];
        for (int shift = 0; shift < 32; shift += 8) {
            int[] count = new int[256];
            for (long x : a) count[(int) ((x >> shift) & 0xFF)]++;
            int sum = 0;
            for (int i = 0; i < 256; i++) { int c = count[i]; count[i] = sum; sum += c; }
            for (long x : a) tmp[count[(int) ((x >> shift) & 0xFF)]++] = x;
            System.arraycopy(tmp, 0, a, 0, a.length);
        }
    }

    public static void main(String[] args) {
        long[] a = {999999999L, 0L, 123456789L, 42L, 1000000000L, 7L, 500000000L};
        radixSort(a);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < a.length; i++) sb.append(a[i]).append(i + 1 < a.length ? ", " : "");
        System.out.println(sb.append("]"));
    }
}
