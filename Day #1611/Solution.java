// Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
// k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).
import java.util.Arrays;

public class Solution {
    static String smallest(String s, int k) {
        if (k >= 2) {
            char[] c = s.toCharArray();
            Arrays.sort(c);
            return new String(c);
        }
        // k == 1: smallest rotation
        String best = s;
        int n = s.length();
        for (int i = 1; i < n; i++) {
            String rot = s.substring(i) + s.substring(0, i);
            if (rot.compareTo(best) < 0) best = rot;
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(smallest("daily", 1));
    }
}
