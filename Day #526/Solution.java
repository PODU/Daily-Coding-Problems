// k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
// of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.
import java.util.Arrays;

public class Solution {
    static String smallest(String s, int k) {
        if (k >= 2) {
            char[] c = s.toCharArray();
            Arrays.sort(c);
            return new String(c);
        }
        int n = s.length();
        String best = s;
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
