// Smallest window containing every distinct character of the string.
// Sliding window with frequency counts; expand then shrink. O(n) time, O(k) space.
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
        return best;
    }

    public static void main(String[] args) {
        String s = "jiujitsu";
        System.out.println(smallestWindow(s));
    }
}
