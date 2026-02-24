// Day 1124 - Minimum points to stab all intervals
// Greedy: sort by right endpoint, place a point at the end of each not-yet-
// stabbed interval. Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    static List<Integer> stab(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] iv : intervals)
            if (iv[0] > last) { last = iv[1]; points.add(iv[1]); }
        return points;
    }

    public static void main(String[] args) {
        int[][] intervals = {{1, 4}, {4, 5}, {7, 9}, {9, 12}};
        System.out.println(stab(intervals)); // [4, 9]
    }
}
