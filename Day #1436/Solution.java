// Day 1436: Length of longest subarray with all distinct elements.
// Approach: Sliding window with last-seen index map; shrink left on repeat.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static int longestDistinct(int[] arr) {
        Map<Integer,Integer> last = new HashMap<>();
        int best = 0, left = 0;
        for (int right = 0; right < arr.length; right++) {
            Integer prev = last.get(arr[right]);
            if (prev != null && prev >= left) left = prev + 1;
            last.put(arr[right], right);
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] arr = {5, 1, 3, 5, 2, 3, 4, 1};
        System.out.println(longestDistinct(arr)); // 5
    }
}
