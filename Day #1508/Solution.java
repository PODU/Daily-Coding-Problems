// Product of array except self without division: prefix-product then suffix-product.
// Two passes, output array only. Time O(n), Space O(1) extra (besides output).
public class Solution {
    static long[] productExceptSelf(int[] nums) {
        int n = nums.length;
        long[] res = new long[n];
        res[0] = 1;
        for (int i = 1; i < n; i++) res[i] = res[i - 1] * nums[i - 1];
        long suffix = 1;
        for (int i = n - 1; i >= 0; i--) {
            res[i] *= suffix;
            suffix *= nums[i];
        }
        return res;
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3, 4, 5};
        long[] res = productExceptSelf(nums);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append(res[i]);
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
