// Day 283: First N regular (5-smooth) numbers via 3-pointer merge of 2x,3x,5x.
// Time O(N), Space O(N).
public class Solution {
    static long[] regular(int n) {
        long[] h = new long[n];
        h[0] = 1;
        int i2 = 0, i3 = 0, i5 = 0;
        for (int i = 1; i < n; i++) {
            long n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
            long nxt = Math.min(n2, Math.min(n3, n5));
            h[i] = nxt;
            if (nxt == n2) i2++;
            if (nxt == n3) i3++;
            if (nxt == n5) i5++;
        }
        return h;
    }

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        for (long x : regular(10)) sb.append(x).append(" ");
        System.out.println(sb.toString().trim()); // 1 2 3 4 5 6 8 9 10 12
    }
}
