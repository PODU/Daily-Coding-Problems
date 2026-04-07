// Move one of first k chars to the end, unlimited times.
// k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
// Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.
import java.util.*;

public class Solution {
    static String smallestString(String s, int k) {
        if (k >= 2) {
            char[] c = s.toCharArray();
            Arrays.sort(c);
            return new String(c);
        }
        String best = s;
        for (int i = 1; i < s.length(); i++) {
            String rot = s.substring(i) + s.substring(0, i);
            if (rot.compareTo(best) < 0) best = rot;
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(smallestString("daily", 1)); // ailyd
    }
}
