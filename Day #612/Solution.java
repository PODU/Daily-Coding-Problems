// Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
// Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).
import java.util.*;

public class Solution {
    static int minRemovals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        int kept = 0, end = Integer.MIN_VALUE;
        for (int[] iv : intervals)
            if (iv[0] >= end) { kept++; end = iv[1]; }
        return intervals.length - kept;
    }

    public static void main(String[] args) {
        int[][] intervals = {{7, 9}, {2, 4}, {5, 8}};
        System.out.println(minRemovals(intervals)); // 1
    }
}
