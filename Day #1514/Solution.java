// Sliding window with last-seen index map; advance left past duplicates, track max window length.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static int longestDistinct(int[] a) {
        Map<Integer, Integer> last = new HashMap<>();
        int best = 0, left = 0;
        for (int r = 0; r < a.length; r++) {
            Integer prev = last.get(a[r]);
            if (prev != null && prev >= left) left = prev + 1;
            last.put(a[r], r);
            best = Math.max(best, r - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {5, 1, 3, 5, 2, 3, 4, 1};
        System.out.println(longestDistinct(a));
    }
}
