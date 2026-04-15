// Longest contiguous subarray with at most two distinct values.
// Sliding window + hashmap of counts, shrink when distinct > 2. Time O(n), Space O(1).
import java.util.HashMap;

public class Solution {
    static int longestTwoDistinct(int[] a) {
        HashMap<Integer, Integer> cnt = new HashMap<>();
        int left = 0, best = 0;
        for (int right = 0; right < a.length; right++) {
            cnt.merge(a[right], 1, Integer::sum);
            while (cnt.size() > 2) {
                int c = cnt.get(a[left]) - 1;
                if (c == 0) cnt.remove(a[left]);
                else cnt.put(a[left], c);
                left++;
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {2, 1, 2, 3, 3, 1, 3, 5};
        System.out.println(longestTwoDistinct(a));
    }
}
