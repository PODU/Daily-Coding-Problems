// Day 1204: GCD of n numbers.
// Fold Euclidean gcd across the list. Time O(n log max), Space O(1).
public class Solution {
    static long gcd(long a, long b) { return b == 0 ? a : gcd(b, a % b); }

    static long gcdAll(long[] a) {
        long g = 0;
        for (long x : a) g = gcd(g, x);
        return g;
    }

    public static void main(String[] args) {
        System.out.println(gcdAll(new long[]{42, 56, 14})); // 14
    }
}
