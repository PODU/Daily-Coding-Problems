// Longest Increasing Subsequence (length), patience sorting.
// Maintain tails[]; binary-search insertion point for each value. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static int lengthOfLIS(int[] nums) {
        List<Integer> tails = new ArrayList<>();
        for (int x : nums) {
            int lo = 0, hi = tails.size();
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (tails.get(mid) < x) lo = mid + 1;
                else hi = mid;
            }
            if (lo == tails.size()) tails.add(x);
            else tails.set(lo, x);
        }
        return tails.size();
    }

    public static void main(String[] args) {
        int[] nums = {0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15};
        System.out.println(lengthOfLIS(nums));
    }
}
