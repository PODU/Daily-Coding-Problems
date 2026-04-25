// Day 1421: product of all elements except self, without division.
// Approach: prefix products pass then suffix products pass. O(n) time, O(1) extra (besides output).
import java.util.*;

public class Solution {
    static long[] productExceptSelf(int[] nums) {
        int n = nums.length;
        long[] res = new long[n];
        long prefix = 1;
        for (int i = 0; i < n; i++) { res[i] = prefix; prefix *= nums[i]; }
        long suffix = 1;
        for (int i = n - 1; i >= 0; i--) { res[i] *= suffix; suffix *= nums[i]; }
        return res;
    }

    static void show(int[] in) {
        System.out.println(Arrays.toString(productExceptSelf(in)));
    }

    public static void main(String[] args) {
        show(new int[]{1, 2, 3, 4, 5}); // [120, 60, 40, 30, 24]
        show(new int[]{3, 2, 1});       // [2, 3, 6]
    }
}
