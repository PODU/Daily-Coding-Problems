// Two single elements (rest in pairs): XOR all -> a^b, split by a set bit, XOR groups.
// Time: O(n), Space: O(1).
public class Solution {
    public static long[] twoUnique(long[] arr) {
        long x = 0;
        for (long v : arr) x ^= v;
        long bit = x & (-x); // lowest set bit
        long a = 0, b = 0;
        for (long v : arr) {
            if ((v & bit) != 0) a ^= v;
            else b ^= v;
        }
        if (a > b) { long t = a; a = b; b = t; }
        return new long[]{a, b};
    }

    public static void main(String[] args) {
        long[] arr = {2, 4, 6, 8, 10, 2, 6, 10};
        long[] r = twoUnique(arr);
        System.out.println(r[0] + " and " + r[1]);
    }
}
