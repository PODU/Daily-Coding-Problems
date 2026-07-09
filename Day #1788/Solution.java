// k nearest points: stable sort by squared Euclidean distance, take first k.
// Time O(N log N), Space O(N).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] pts = {{0,0},{5,4},{3,1}};
        int cx = 1, cy = 2, k = 2;
        Integer[] idx = new Integer[pts.length];
        for (int i = 0; i < idx.length; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> {
            long da = (long)(pts[a][0]-cx)*(pts[a][0]-cx)+(long)(pts[a][1]-cy)*(pts[a][1]-cy);
            long db = (long)(pts[b][0]-cx)*(pts[b][0]-cx)+(long)(pts[b][1]-cy)*(pts[b][1]-cy);
            return Long.compare(da, db);
        }); // Arrays.sort on objects is stable
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < k; i++) {
            if (i > 0) sb.append(", ");
            sb.append("(").append(pts[idx[i]][0]).append(", ").append(pts[idx[i]][1]).append(")");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
