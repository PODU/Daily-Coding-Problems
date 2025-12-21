// Day 778: Interleave ranked preference lists -> topological sort (Kahn's).
// Consecutive items in each list become edges. FIFO queue + first-seen order.
// O(V + E).
import java.util.*;

public class Solution {
    static List<Integer> interleave(int[][] lists) {
        List<Integer> order = new ArrayList<>();
        Set<Integer> seen = new HashSet<>();
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Map<Integer, Integer> indeg = new HashMap<>();
        for (int[] l : lists) {
            for (int x : l) if (seen.add(x)) { order.add(x); indeg.putIfAbsent(x, 0); }
            for (int i = 0; i + 1 < l.length; i++) {
                adj.computeIfAbsent(l[i], k -> new ArrayList<>()).add(l[i + 1]);
                indeg.merge(l[i + 1], 1, Integer::sum);
            }
        }
        Deque<Integer> q = new ArrayDeque<>();
        for (int x : order) if (indeg.get(x) == 0) q.add(x);
        List<Integer> res = new ArrayList<>();
        while (!q.isEmpty()) {
            int u = q.poll();
            res.add(u);
            for (int v : adj.getOrDefault(u, Collections.emptyList()))
                if (indeg.merge(v, -1, Integer::sum) == 0) q.add(v);
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
        System.out.println(interleave(lists)); // [2, 1, 6, 7, 3, 9, 5]
    }
}
