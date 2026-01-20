// Day 931: GCD of n numbers by folding pairwise gcd.
// Time: O(n * log(maxVal)), Space: O(1).
public class Solution {
    static long gcd(long a, long b) {
        while (b != 0) { long t = a % b; a = b; b = t; }
        return a;
    }
    static long gcdList(long[] nums) {
        long g = 0;
        for (long x : nums) g = gcd(g, x);
        return g;
    }
    public static void main(String[] args) {
        long[] nums = {42, 56, 14};
        System.out.println(gcdList(nums)); // 14
    }
}
