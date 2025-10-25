// Day 489: Longest subarray with all distinct elements.
// Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static int longestDistinctSubarray(int[] a) {
        Map<Integer, Integer> last = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < a.length; right++) {
            Integer prev = last.get(a[right]);
            if (prev != null && prev >= left) left = prev + 1;
            last.put(a[right], right);
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {5, 1, 3, 5, 2, 3, 4, 1};
        System.out.println(longestDistinctSubarray(a)); // 5
    }
}
