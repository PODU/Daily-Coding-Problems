// Day 189: Longest contiguous subarray with all distinct elements.
// Sliding window with last-seen map. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static int longestDistinct(int[] a) {
        Map<Integer, Integer> last = new HashMap<>();
        int best = 0, start = 0;
        for (int i = 0; i < a.length; i++) {
            Integer p = last.get(a[i]);
            if (p != null && p >= start) start = p + 1;
            last.put(a[i], i);
            best = Math.max(best, i - start + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(longestDistinct(new int[]{5, 1, 3, 5, 2, 3, 4, 1}));
    }
}
