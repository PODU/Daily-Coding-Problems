// Skyline via sweep-line + max-heap (TreeMap as multiset). Emit point when max height changes.
// Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] buildings = {{0,15,3},{4,11,5},{19,23,4}};
        // events: {x, signed height} with start = -h, end = +h
        List<int[]> events = new ArrayList<>();
        for (int[] b : buildings) {
            events.add(new int[]{b[0], -b[2]});
            events.add(new int[]{b[1],  b[2]});
        }
        events.sort((p, q) -> p[0] != q[0] ? p[0] - q[0] : p[1] - q[1]);

        TreeMap<Integer,Integer> heights = new TreeMap<>();
        heights.put(0, 1);
        int prev = 0;
        List<int[]> res = new ArrayList<>();
        for (int[] e : events) {
            int x = e[0], h = e[1];
            if (h < 0) heights.merge(-h, 1, Integer::sum);
            else {
                int c = heights.get(h);
                if (c == 1) heights.remove(h); else heights.put(h, c - 1);
            }
            int cur = heights.lastKey();
            if (cur != prev) {
                res.add(new int[]{x, cur});
                prev = cur;
            }
        }

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
