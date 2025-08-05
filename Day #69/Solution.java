// Largest product of three: max(max1*max2*max3, min1*min2*max1) tracking top-3 & bottom-2. Time O(n), Space O(1).
public class Solution {
    static long largestTripleProduct(int[] nums) {
        long max1 = Long.MIN_VALUE, max2 = Long.MIN_VALUE, max3 = Long.MIN_VALUE;
        long min1 = Long.MAX_VALUE, min2 = Long.MAX_VALUE;
        for (int x : nums) {
            if (x > max1) { max3 = max2; max2 = max1; max1 = x; }
            else if (x > max2) { max3 = max2; max2 = x; }
            else if (x > max3) { max3 = x; }
            if (x < min1) { min2 = min1; min1 = x; }
            else if (x < min2) { min2 = x; }
        }
        return Math.max(max1 * max2 * max3, min1 * min2 * max1);
    }

    public static void main(String[] args) {
        int[] nums = {-10, -10, 5, 2};
        System.out.println(largestTripleProduct(nums));
    }
}
