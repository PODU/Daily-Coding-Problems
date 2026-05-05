// Day 1472: Largest product of any three integers.
// Track 3 largest and 2 smallest in one pass; max of two candidate products.
// Time O(N), Space O(1).
public class Solution {
    static long maxProductThree(long[] nums) {
        long max1 = Long.MIN_VALUE, max2 = Long.MIN_VALUE, max3 = Long.MIN_VALUE;
        long min1 = Long.MAX_VALUE, min2 = Long.MAX_VALUE;
        for (long n : nums) {
            if (n > max1) { max3 = max2; max2 = max1; max1 = n; }
            else if (n > max2) { max3 = max2; max2 = n; }
            else if (n > max3) { max3 = n; }
            if (n < min1) { min2 = min1; min1 = n; }
            else if (n < min2) { min2 = n; }
        }
        return Math.max(max1 * max2 * max3, max1 * min1 * min2);
    }

    public static void main(String[] args) {
        System.out.println(maxProductThree(new long[]{-10, -10, 5, 2}));  // 500
    }
}
