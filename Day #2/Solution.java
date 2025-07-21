// Product of array except self via prefix and suffix passes (no division).
// Time: O(n), Space: O(1) extra (excluding output).
import java.util.Arrays;

public class Solution {
    static long[] productExceptSelf(int[] nums) {
        int n = nums.length;
        long[] res = new long[n];
        long pre = 1;
        for (int i = 0; i < n; i++) { res[i] = pre; pre *= nums[i]; }
        long suf = 1;
        for (int i = n - 1; i >= 0; i--) { res[i] *= suf; suf *= nums[i]; }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(productExceptSelf(new int[]{1, 2, 3, 4, 5})));
    }
}
