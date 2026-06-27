// Min intervals to remove for non-overlapping (touching allowed).
// Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.
import java.util.*;

public class Solution {
    static int minRemovals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        int kept = 0, lastEnd = Integer.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] >= lastEnd) { kept++; lastEnd = iv[1]; }
        }
        return intervals.length - kept;
    }

    public static void main(String[] args) {
        int[][] intervals = {{7, 9}, {2, 4}, {5, 8}};
        System.out.println(minRemovals(intervals));
    }
}
