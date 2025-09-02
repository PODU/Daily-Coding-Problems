// Day 200: Minimum points stabbing all intervals.
// Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
// Time: O(n log n), Space: O(1).
import java.util.*;

public class Solution {
    static List<Integer> stab(int[][] iv) {
        Arrays.sort(iv, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> pts = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] p : iv) {
            if (p[0] > last) { last = p[1]; pts.add((int) last); }
        }
        return pts;
    }

    public static void main(String[] args) {
        int[][] iv = {{1, 4}, {4, 5}, {7, 9}, {9, 12}};
        System.out.println(stab(iv)); // [4, 9]
    }
}
