// Smallest window containing every distinct char: sliding window with two pointers.
// Expand right until all distinct chars present, shrink left to minimize. Time O(n), space O(k).
import java.util.*;

public class Solution {
    static int smallestWindow(String s) {
        Set<Character> distinct = new HashSet<>();
        for (char c : s.toCharArray()) distinct.add(c);
        int need = distinct.size();
        Map<Character, Integer> cnt = new HashMap<>();
        int have = 0, best = Integer.MAX_VALUE, left = 0;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.merge(c, 1, Integer::sum);
            if (cnt.get(c) == 1) have++;
            while (have == need) {
                best = Math.min(best, right - left + 1);
                char lc = s.charAt(left);
                cnt.merge(lc, -1, Integer::sum);
                if (cnt.get(lc) == 0) have--;
                left++;
            }
        }
        return best == Integer.MAX_VALUE ? 0 : best;
    }

    public static void main(String[] args) {
        System.out.println(smallestWindow("jiujitsu"));
    }
}
