// Longest strictly Increasing Subsequence length via patience sorting + binary search.
// Time O(N log N), Space O(N). Also reconstructs one valid LIS.
import java.util.*;

public class Solution {
    static int[] lisTails;   // not used as field; kept local below

    static Object[] lisLength(int[] nums) {
        int n = nums.length;
        int[] tails = new int[n];     // value of smallest tail per length
        int[] tailsIdx = new int[n];  // index in nums of that tail
        int len = 0;
        int[] prev = new int[n];
        Arrays.fill(prev, -1);
        for (int i = 0; i < n; i++) {
            int x = nums[i];
            // lower_bound over tails[0..len)
            int lo = 0, hi = len;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (tails[mid] < x) lo = mid + 1;
                else hi = mid;
            }
            int pos = lo;
            tails[pos] = x;
            tailsIdx[pos] = i;
            if (pos == len) len++;
            prev[i] = pos > 0 ? tailsIdx[pos - 1] : -1;
        }
        ArrayList<Integer> seq = new ArrayList<>();
        int k = len > 0 ? tailsIdx[len - 1] : -1;
        while (k != -1) {
            seq.add(nums[k]);
            k = prev[k];
        }
        Collections.reverse(seq);
        return new Object[]{len, seq};
    }

    public static void main(String[] args) {
        int[] nums = {0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15};
        Object[] res = lisLength(nums);
        int len = (Integer) res[0];
        @SuppressWarnings("unchecked")
        ArrayList<Integer> seq = (ArrayList<Integer>) res[1];
        System.out.println(len);
        System.out.println(seq);
    }
}
