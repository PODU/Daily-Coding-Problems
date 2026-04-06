// Smallest sparse number (no adjacent 1 bits) >= N. Scan bits low->high; at the
// top of each adjacent-ones run, carry up and clear below. Time O(log N).
import java.util.*;

public class Solution {
    static long nextSparse(long x) {
        if (x == 0) return 0;
        ArrayList<Integer> b = new ArrayList<>();
        for (long t = x; t != 0; t >>= 1) b.add((int) (t & 1));
        b.add(0); b.add(0); // padding for carries
        int n = b.size();
        for (int i = 1; i < n - 1; i++) {
            if (b.get(i) == 1 && b.get(i - 1) == 1 && b.get(i + 1) == 0) {
                b.set(i + 1, 1);
                for (int j = i; j >= 0; j--) b.set(j, 0);
            }
        }
        long ans = 0;
        for (int i = 0; i < n; i++) if (b.get(i) == 1) ans |= (1L << i);
        return ans;
    }

    public static void main(String[] args) {
        System.out.println(nextSparse(22)); // 32
    }
}
