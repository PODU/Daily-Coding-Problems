// Day 1477: First N regular numbers (only prime factors 2, 3, 5).
// DP with three pointers merging *2, *3, *5 sequences. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static long[] regularNumbers(int n) {
        if (n <= 0) return new long[0];
        long[] h = new long[n];
        h[0] = 1;
        int i2 = 0, i3 = 0, i5 = 0;
        for (int k = 1; k < n; ++k) {
            long nxt = Math.min(h[i2] * 2, Math.min(h[i3] * 3, h[i5] * 5));
            h[k] = nxt;
            if (nxt == h[i2] * 2) ++i2;
            if (nxt == h[i3] * 3) ++i3;
            if (nxt == h[i5] * 5) ++i5;
        }
        return h;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(regularNumbers(10)));
        // [1, 2, 3, 4, 5, 6, 8, 9, 10, 12]
    }
}
