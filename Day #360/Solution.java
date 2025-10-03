// Merge ranked lists via topological sort: edge a->b for consecutive a,b in any list; Kahn's with FIFO queue. O(V+E).
import java.util.*;

public class Solution {
    static List<Integer> merge(int[][] lists) {
        List<Integer> order = new ArrayList<>();      // first-appearance order
        Set<Integer> seen = new HashSet<>();
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Map<Integer, Integer> indeg = new HashMap<>();
        for (int[] l : lists) {
            for (int x : l) {
                if (seen.add(x)) { order.add(x); indeg.putIfAbsent(x, 0); }
            }
            for (int i = 0; i + 1 < l.length; i++) {
                adj.computeIfAbsent(l[i], k -> new ArrayList<>()).add(l[i + 1]);
                indeg.merge(l[i + 1], 1, Integer::sum);
            }
        }
        Deque<Integer> q = new ArrayDeque<>();
        for (int x : order) if (indeg.get(x) == 0) q.addLast(x);
        List<Integer> res = new ArrayList<>();
        while (!q.isEmpty()) {
            int u = q.pollFirst();
            res.add(u);
            for (int v : adj.getOrDefault(u, Collections.emptyList())) {
                indeg.merge(v, -1, Integer::sum);
                if (indeg.get(v) == 0) q.addLast(v);
            }
        }
        return res;
    }
    public static void main(String[] args) {
        int[][] lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
        System.out.println(merge(lists));
    }
}
