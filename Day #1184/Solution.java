// Day 1184: Interleave ranked lists into one playlist respecting every ordering.
// Build a DAG of consecutive-preference edges and run Kahn topological sort (FIFO).
// Time O(V + E), Space O(V + E).
import java.util.*;

public class Solution {
    static List<Integer> interleave(int[][] lists) {
        List<Integer> order = new ArrayList<>();
        Set<Integer> known = new HashSet<>();
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Map<Integer, Integer> indeg = new HashMap<>();
        Set<Long> edges = new HashSet<>();

        for (int[] l : lists) {
            for (int v : l) if (known.add(v)) { order.add(v); indeg.putIfAbsent(v, 0); adj.putIfAbsent(v, new ArrayList<>()); }
            for (int i = 0; i + 1 < l.length; i++) {
                int u = l[i], w = l[i + 1];
                long key = (long) u << 32 | (w & 0xffffffffL);
                if (u != w && edges.add(key)) {
                    adj.get(u).add(w);
                    indeg.merge(w, 1, Integer::sum);
                }
            }
        }

        Deque<Integer> q = new ArrayDeque<>();
        for (int v : order) if (indeg.get(v) == 0) q.add(v);
        List<Integer> res = new ArrayList<>();
        while (!q.isEmpty()) {
            int v = q.poll();
            res.add(v);
            for (int w : adj.get(v)) if (indeg.merge(w, -1, Integer::sum) == 0) q.add(w);
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
        System.out.println(interleave(lists)); // [2, 1, 6, 7, 3, 9, 5]
    }
}
