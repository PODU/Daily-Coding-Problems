// Day 424: Two unique elements via XOR partition. O(n) time, O(1) space.
// XOR all -> a^b; isolate a low set bit; partition & XOR each group -> a, b.
public class Solution {
    public static void main(String[] args) {
        long[] a = {2, 4, 6, 8, 10, 2, 6, 10};
        long x = 0;
        for (long v : a) x ^= v;
        long bit = x & (-x);
        long p = 0, q = 0;
        for (long v : a) {
            if ((v & bit) != 0) p ^= v;
            else q ^= v;
        }
        long lo = Math.min(p, q), hi = Math.max(p, q);
        System.out.println(lo + " and " + hi);
    }
}
