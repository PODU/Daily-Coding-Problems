// First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static long[] regular(int n) {
        long[] h = new long[n];
        h[0] = 1;
        int i2 = 0, i3 = 0, i5 = 0;
        for (int i = 1; i < n; i++) {
            long n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
            long m = Math.min(n2, Math.min(n3, n5));
            h[i] = m;
            if (m == n2) i2++;
            if (m == n3) i3++;
            if (m == n5) i5++;
        }
        return h;
    }

    public static void main(String[] args) {
        long[] res = regular(10);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) { sb.append(res[i]); if (i + 1 < res.length) sb.append(", "); }
        sb.append("]");
        System.out.println(sb);
    }
}
