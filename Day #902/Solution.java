// Greedy: sort intervals by end; keep interval if start >= last kept end.
// Answer = total - kept. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static int eraseOverlapIntervals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        int kept = 0, lastEnd = Integer.MIN_VALUE;
        for (int[] it : intervals) {
            if (it[0] >= lastEnd) { kept++; lastEnd = it[1]; }
        }
        return intervals.length - kept;
    }

    public static void main(String[] args) {
        int[][] intervals = {{7,9},{2,4},{5,8}};
        System.out.println(eraseOverlapIntervals(intervals));
    }
}
