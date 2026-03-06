// Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.
import java.util.*;

public class Solution {
    static int eraseOverlapIntervals(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        int removals = 0;
        long lastEnd = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] >= lastEnd) {
                lastEnd = iv[1];
            } else {
                removals++;
            }
        }
        return removals;
    }

    public static void main(String[] args) {
        int[][] intervals = {{7, 9}, {2, 4}, {5, 8}};
        System.out.println(eraseOverlapIntervals(intervals));
    }
}
