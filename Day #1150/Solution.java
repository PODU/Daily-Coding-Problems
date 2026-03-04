// Day 1150: Skyline - sweep line over building edges with a height multiset (TreeMap).
// Track current max height; emit point when it changes. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static List<int[]> skyline(int[][] bld) {
        List<int[]> events = new ArrayList<>();
        for (int[] b : bld) { events.add(new int[]{b[0], -b[2]}); events.add(new int[]{b[1], b[2]}); }
        events.sort((a, c) -> a[0] != c[0] ? a[0] - c[0] : a[1] - c[1]);
        TreeMap<Integer, Integer> heights = new TreeMap<>();
        heights.put(0, 1);
        int prev = 0;
        List<int[]> res = new ArrayList<>();
        for (int[] e : events) {
            int h = e[1];
            if (h < 0) heights.merge(-h, 1, Integer::sum);
            else {
                int cnt = heights.get(h);
                if (cnt == 1) heights.remove(h); else heights.put(h, cnt - 1);
            }
            int cur = heights.lastKey();
            if (cur != prev) { res.add(new int[]{e[0], cur}); prev = cur; }
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] bld = {{0, 15, 3}, {4, 11, 5}, {19, 23, 4}};
        List<int[]> sk = skyline(bld);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < sk.size(); i++)
            sb.append("(").append(sk.get(i)[0]).append(", ").append(sk.get(i)[1]).append(")")
              .append(i + 1 < sk.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb); // [(0, 3), (4, 5), (11, 3), (15, 0), (19, 4), (23, 0)]
    }
}
