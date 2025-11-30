// Day 674: Longest contiguous subarray with at most 2 distinct values. Sliding window.
// Time O(n), Space O(1) (at most 3 keys in the map).
import java.util.*;

public class Solution {
    static int longestTwoTypes(int[] a) {
        Map<Integer, Integer> cnt = new HashMap<>();
        int best = 0, l = 0;
        for (int r = 0; r < a.length; r++) {
            cnt.merge(a[r], 1, Integer::sum);
            while (cnt.size() > 2) {
                if (cnt.merge(a[l], -1, Integer::sum) == 0) cnt.remove(a[l]);
                l++;
            }
            best = Math.max(best, r - l + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {2, 1, 2, 3, 3, 1, 3, 5};
        System.out.println(longestTwoTypes(a)); // 4
    }
}
