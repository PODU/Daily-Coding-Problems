// LSD radix sort (base 256, 4 passes over 32-bit ints). O(N*d)~O(N) time, O(N) space.
// Avoids a 1e9-size counting array; memory scales with N, not value range. Else: external merge sort.
public class Solution {
    static void radixSort(long[] a) {
        int n = a.length;
        long[] buf = new long[n];
        for (int shift = 0; shift < 32; shift += 8) {
            int[] count = new int[256];
            for (long v : a) count[(int)((v >> shift) & 0xFF)]++;
            int sum = 0;
            for (int i = 0; i < 256; i++) { int c = count[i]; count[i] = sum; sum += c; }
            for (long v : a) buf[count[(int)((v >> shift) & 0xFF)]++] = v;
            System.arraycopy(buf, 0, a, 0, n);
        }
    }

    public static void main(String[] args) {
        long[] a = {829, 3, 1000000000L, 42, 17, 999, 256, 0, 524287, 42};
        radixSort(a);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < a.length; i++) { if (i > 0) sb.append(' '); sb.append(a[i]); }
        System.out.println(sb);
    }
}
