// Day 184: GCD of n numbers via iterated Euclidean algorithm.
// Time O(n * log(max)), Space O(1).
public class Solution {
    static long gcd2(long a, long b) { while (b != 0) { long t = a % b; a = b; b = t; } return a; }

    static long gcdAll(long[] nums) {
        long g = 0;
        for (long x : nums) g = gcd2(g, x);
        return g;
    }

    public static void main(String[] args) {
        long[] nums = {42, 56, 14};
        System.out.println(gcdAll(nums));
    }
}
