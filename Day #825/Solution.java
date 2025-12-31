// Sorted squares via two-pointer merge from both ends, filling result from the back.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static long[] sortedSquares(long[] nums) {
        int n = nums.length;
        long[] res = new long[n];
        int lo = 0, hi = n - 1;
        for (int k = n - 1; k >= 0; k--) {
            long l = nums[lo] * nums[lo];
            long r = nums[hi] * nums[hi];
            if (l > r) { res[k] = l; lo++; }
            else       { res[k] = r; hi--; }
        }
        return res;
    }

    public static void main(String[] args) {
        long[] nums = {-9, -2, 0, 2, 3};
        long[] res = sortedSquares(nums);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append(res[i]);
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
