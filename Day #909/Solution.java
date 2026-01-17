// Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
// group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static List<Integer> minPoints(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        boolean have = false;
        long anchorEnd = Long.MIN_VALUE, groupMaxStart = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (!have || iv[0] > anchorEnd) {
                if (have) points.add((int) groupMaxStart);
                have = true; anchorEnd = iv[1]; groupMaxStart = iv[0];
            } else if (iv[0] > groupMaxStart) {
                groupMaxStart = iv[0];
            }
        }
        if (have) points.add((int) groupMaxStart);
        return points;
    }

    public static void main(String[] args) {
        int[][] intervals = {{0,3},{2,6},{3,4},{6,9}};
        List<Integer> pts = minPoints(intervals);
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < pts.size(); i++) {
            sb.append(pts.get(i));
            if (i + 1 < pts.size()) sb.append(", ");
        }
        sb.append("}");
        System.out.println(sb);
    }
}
