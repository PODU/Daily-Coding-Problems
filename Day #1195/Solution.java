// Smallest window containing all distinct chars of the string. Sliding window:
// expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.
import java.util.*;

public class Solution {
    static int smallestWindow(String s) {
        Set<Character> set = new HashSet<>();
        for (char c : s.toCharArray()) set.add(c);
        int distinct = set.size();

        Map<Character, Integer> cnt = new HashMap<>();
        int have = 0, left = 0, best = Integer.MAX_VALUE;
        for (int right = 0; right < s.length(); right++) {
            char c = s.charAt(right);
            cnt.merge(c, 1, Integer::sum);
            if (cnt.get(c) == 1) have++;
            while (have == distinct) {
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
