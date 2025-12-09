// Day 723: Smallest set of points stabbing all closed intervals.
// Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
// Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)
import java.util.*;

public class Solution {
    static List<Integer> minStabbingPoints(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] iv : intervals)
            if (iv[0] > last) { last = iv[1]; points.add(iv[1]); }
        return points;
    }

    public static void main(String[] args) {
        int[][] intervals = {{0,3},{2,6},{3,4},{6,9}};
        List<Integer> pts = minStabbingPoints(intervals);
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < pts.size(); i++)
            sb.append(pts.get(i)).append(i + 1 < pts.size() ? ", " : "");
        sb.append("}");
        System.out.println(sb);
    }
}
