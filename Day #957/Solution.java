// Day 957: skyline problem via sweep line with a TreeMap of active heights.
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static List<int[]> skyline(int[][] buildings) {
        List<int[]> events = new ArrayList<>();
        for (int[] b : buildings) {
            events.add(new int[]{b[0], -b[2]});
            events.add(new int[]{b[1], b[2]});
        }
        events.sort((p, q) -> p[0] != q[0] ? p[0] - q[0] : p[1] - q[1]);
        TreeMap<Integer, Integer> active = new TreeMap<>();
        active.put(0, 1);
        int prev = 0;
        List<int[]> res = new ArrayList<>();
        for (int[] e : events) {
            int h = e[1];
            if (h < 0) active.merge(-h, 1, Integer::sum);
            else {
                int c = active.get(h);
                if (c == 1) active.remove(h); else active.put(h, c - 1);
            }
            int cur = active.lastKey();
            if (cur != prev) { res.add(new int[]{e[0], cur}); prev = cur; }
        }
        return res;
    }

    public static void main(String[] args) {
        List<int[]> res = skyline(new int[][]{{0,15,3},{4,11,5},{19,23,4}});
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
        }
        sb.append("]");
        System.out.println(sb); // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
    }
}
