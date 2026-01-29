// Longest Increasing Subsequence via patience sorting: maintain "tails" array,
// binary-search the insertion point for each element. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int lengthOfLIS(int[] nums) {
        int[] tails = new int[nums.length];
        int size = 0;
        for (int x : nums) {
            int lo = 0, hi = size;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (tails[mid] < x) lo = mid + 1;
                else hi = mid;
            }
            tails[lo] = x;
            if (lo == size) size++;
        }
        return size;
    }

    public static void main(String[] args) {
        int[] nums = {10, 9, 2, 5, 3, 7, 101, 18};
        System.out.println("Input: [10, 9, 2, 5, 3, 7, 101, 18]");
        System.out.println("LIS length: " + lengthOfLIS(nums)); // 4
    }
}
