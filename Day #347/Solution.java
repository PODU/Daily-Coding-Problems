// Day 347: Lexicographically smallest string by moving one of first k letters to the end.
// k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
import java.util.*;

public class Solution {
    static String smallest(String s, int k) {
        if (k == 1) {
            String best = s;
            for (int i = 1; i < s.length(); i++) {
                String rot = s.substring(i) + s.substring(0, i);
                if (rot.compareTo(best) < 0) best = rot;
            }
            return best;
        }
        char[] c = s.toCharArray();
        Arrays.sort(c);
        return new String(c);
    }

    public static void main(String[] args) {
        System.out.println(smallest("daily", 1));
    }
}
