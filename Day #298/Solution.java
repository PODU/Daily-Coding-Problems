// Longest contiguous subarray with at most 2 distinct values via sliding window + count map.
// Time: O(n), Space: O(1) (at most 3 keys in map).
import java.util.*;

public class Solution {
    static int longestAtMost2(int[] a) {
        Map<Integer,Integer> cnt = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < a.length; right++) {
            cnt.merge(a[right], 1, Integer::sum);
            while (cnt.size() > 2) {
                int v = a[left];
                if (cnt.merge(v, -1, Integer::sum) == 0) cnt.remove(v);
                left++;
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {2,1,2,3,3,1,3,5};
        System.out.println(longestAtMost2(a));
    }
}
