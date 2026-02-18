// Day 1097: Smallest string by moving one of first k letters to the end.
// k==1 -> min rotation; k>=2 -> any permutation reachable -> sorted string.
// Time: O(N^2) for k==1, O(N log N) for k>=2. Space: O(N).
import java.util.*;

public class Solution {
    static String smallest(String s, int k) {
        if (k >= 2) {
            char[] a = s.toCharArray();
            Arrays.sort(a);
            return new String(a);
        }
        String best = s, cur = s;
        for (int i = 0; i < s.length(); i++) {
            cur = cur.substring(1) + cur.charAt(0);
            if (cur.compareTo(best) < 0) best = cur;
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(smallest("daily", 1)); // ailyd
    }
}
