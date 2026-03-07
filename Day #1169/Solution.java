// K nearest points: max-heap of size k by squared Euclidean distance,
// then sort the k results by (dist, original index) for deterministic output.
// Time: O(n log k), Space: O(k).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        long[][] points = {{0, 0}, {5, 4}, {3, 1}};
        long cx = 1, cy = 2;
        int k = 2;

        // max-heap on dist2 keeping k smallest; store indices
        PriorityQueue<Integer> heap = new PriorityQueue<>(
            (a, b) -> Long.compare(dist2(points[b], cx, cy), dist2(points[a], cx, cy)));
        for (int i = 0; i < points.length; i++) {
            heap.add(i);
            if (heap.size() > k) heap.poll();
        }

        List<Integer> idx = new ArrayList<>(heap);
        idx.sort((a, b) -> {
            long da = dist2(points[a], cx, cy), db = dist2(points[b], cx, cy);
            if (da != db) return Long.compare(da, db);
            return Integer.compare(a, b);
        });

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < idx.size(); i++) {
            if (i > 0) sb.append(", ");
            long[] p = points[idx.get(i)];
            sb.append("(").append(p[0]).append(", ").append(p[1]).append(")");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }

    static long dist2(long[] p, long cx, long cy) {
        long dx = p[0] - cx, dy = p[1] - cy;
        return dx * dx + dy * dy;
    }
}
