// Day 631: Skyline problem.
// Approach: sweep line over edges + TreeMap multiset of active heights.
// Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    static List<int[]> getSkyline(int[][] buildings) {
        List<int[]> events = new ArrayList<>();
        for (int[] b : buildings) {
            events.add(new int[]{b[0], -b[2]});
            events.add(new int[]{b[1], b[2]});
        }
        events.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        TreeMap<Integer, Integer> heights = new TreeMap<>();
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
        return res;
    }

    public static void main(String[] args) {
        int[][] buildings = {{0,15,3},{4,11,5},{19,23,4}};
        List<int[]> sky = getSkyline(buildings);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < sky.size(); i++) {
            sb.append("(").append(sky.get(i)[0]).append(", ").append(sky.get(i)[1]).append(")");
            if (i + 1 < sky.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
