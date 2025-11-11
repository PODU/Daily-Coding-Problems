// Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Solution {
    static List<Integer> stab(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> Integer.compare(a[1], b[1]));
        List<Integer> points = new ArrayList<>();
        long last = Long.MIN_VALUE;
        for (int[] iv : intervals) {
            if (iv[0] > last) {
                last = iv[1];
                points.add(iv[1]);
            }
        }
        return points;
    }

    public static void main(String[] args) {
        int[][] intervals = {{1, 4}, {4, 5}, {7, 9}, {9, 12}};
        List<Integer> pts = stab(intervals);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < pts.size(); i++) {
            sb.append(pts.get(i));
            if (i + 1 < pts.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
