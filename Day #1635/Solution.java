// Minimum stabbing points: greedy sort intervals by right endpoint; add right
// endpoint when current interval is unstabbed. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] intervals = {{1,4},{4,5},{7,9},{9,12}};
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] > last) { points.add(iv[1]); last = iv[1]; }
        }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < points.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(points.get(i));
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
