// Nearest k points: max-heap of size k by squared distance. Time O(n log k), Space O(k).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] points = {{0,0},{5,4},{3,1}};
        int[] central = {1,2};
        int k = 2;

        // max-heap by squared distance
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> {
            long da = (long)(a[0]-central[0])*(a[0]-central[0]) + (long)(a[1]-central[1])*(a[1]-central[1]);
            long db = (long)(b[0]-central[0])*(b[0]-central[0]) + (long)(b[1]-central[1])*(b[1]-central[1]);
            return Long.compare(db, da);
        });
        for (int[] p : points) {
            heap.offer(p);
            if (heap.size() > k) heap.poll();
        }
        List<int[]> res = new ArrayList<>(heap);
        res.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
