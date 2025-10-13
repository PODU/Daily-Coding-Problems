// Day 421: LSD radix sort (base-256). O(n*w) time, O(n) space.
// 4 byte-passes for 32-bit ints; avoids a billion-element counting array.
public class Solution {
    static long[] radixSort(long[] a) {
        int n = a.length;
        long[] buf = new long[n];
        for (int shift = 0; shift < 32; shift += 8) {
            int[] cnt = new int[257];
            for (int i = 0; i < n; i++) cnt[(int) ((a[i] >>> shift) & 0xFF) + 1]++;
            for (int i = 0; i < 256; i++) cnt[i + 1] += cnt[i];
            for (int i = 0; i < n; i++) buf[cnt[(int) ((a[i] >>> shift) & 0xFF)]++] = a[i];
            long[] t = a; a = buf; buf = t;
        }
        return a;
    }

    public static void main(String[] args) {
        long[] a = {5, 3, 1000000000, 0, 42, 7, 42};
        a = radixSort(a);
        StringBuilder sb = new StringBuilder("Sorted: [");
        for (int i = 0; i < a.length; i++) {
            sb.append(a[i]);
            if (i + 1 < a.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
