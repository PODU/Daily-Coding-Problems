// Day 1446: Minimum set of points hitting every closed interval.
// Greedy: sort by right endpoint; whenever the current interval is unhit, pick
// its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
import java.util.*;

public class Solution {
    static List<Integer> minStabbingSet(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] > last) { points.add(iv[1]); last = iv[1]; }
        }
        return points;
    }

    public static void main(String[] args) {
        int[][] intervals = {{0,3},{2,6},{3,4},{6,9}};
        List<Integer> pts = minStabbingSet(intervals);
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < pts.size(); i++)
            sb.append(pts.get(i)).append(i + 1 < pts.size() ? ", " : "");
        sb.append("}");
        System.out.println(sb); // e.g. {3, 9} -- a valid minimum (so is {3, 6})
    }
}
