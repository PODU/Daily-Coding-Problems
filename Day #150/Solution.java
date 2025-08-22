// K nearest points: max-heap of size k keyed by squared distance. Time O(n log k), Space O(k).
import java.util.*;

public class Solution {
    static int[][] kNearest(int[][] pts, int[] c, int k) {
        // max-heap on squared distance; entry = {dist2, index}
        PriorityQueue<long[]> heap = new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0]));
        for (int i = 0; i < pts.length; i++) {
            long dx = pts[i][0] - c[0], dy = pts[i][1] - c[1];
            heap.offer(new long[]{dx * dx + dy * dy, i});
            if (heap.size() > k) heap.poll();
        }
        List<Integer> idx = new ArrayList<>();
        while (!heap.isEmpty()) idx.add((int) heap.poll()[1]);
        Collections.sort(idx); // keep original order for stable output
        int[][] res = new int[idx.size()][];
        for (int i = 0; i < idx.size(); i++) res[i] = pts[idx.get(i)];
        return res;
    }

    public static void main(String[] args) {
        int[][] pts = {{0, 0}, {5, 4}, {3, 1}};
        int[] c = {1, 2};
        int k = 2;
        int[][] res = kNearest(pts, c, k);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append("(").append(res[i][0]).append(", ").append(res[i][1]).append(")");
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
