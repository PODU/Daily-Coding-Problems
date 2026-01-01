// Longest subarray with all distinct elements.
// Sliding window with last-seen index map. Time: O(N), Space: O(N).
import java.util.HashMap;

public class Solution {
    static int longestDistinct(int[] a) {
        HashMap<Integer, Integer> last = new HashMap<>();
        int best = 0, start = 0;
        for (int i = 0; i < a.length; i++) {
            Integer prev = last.get(a[i]);
            if (prev != null && prev >= start) start = prev + 1;
            last.put(a[i], i);
            best = Math.max(best, i - start + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {5, 1, 3, 5, 2, 3, 4, 1};
        System.out.println(longestDistinct(a));
    }
}
