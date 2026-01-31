// Day 995: Product of array except self, without division.
// Left pass stores prefix products; right pass multiplies by suffix products.
// O(n) time, O(1) extra space (besides output).
import java.util.*;

public class Solution {
    static long[] products(long[] nums) {
        int n = nums.length;
        long[] res = new long[n];
        long left = 1;
        for (int i = 0; i < n; i++) { res[i] = left; left *= nums[i]; }
        long right = 1;
        for (int i = n - 1; i >= 0; i--) { res[i] *= right; right *= nums[i]; }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(products(new long[]{1, 2, 3, 4, 5}))); // [120, 60, 40, 30, 24]
        System.out.println(Arrays.toString(products(new long[]{3, 2, 1})));        // [2, 3, 6]
    }
}
