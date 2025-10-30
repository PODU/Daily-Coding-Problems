// Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
import java.util.*;

public class Solution {
    static int longestConsecutive(int[] a) {
        Set<Integer> s = new HashSet<>();
        for (int x : a) s.add(x);
        int best = 0;
        for (int x : s) {
            if (s.contains(x - 1)) continue;
            int len = 1, cur = x;
            while (s.contains(cur + 1)) { cur++; len++; }
            best = Math.max(best, len);
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(longestConsecutive(new int[]{100, 4, 200, 1, 3, 2}));
    }
}
