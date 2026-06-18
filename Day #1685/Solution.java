// Day 1685: Merge ranked preference lists -> topological sort (Kahn's BFS, FIFO,
// first-seen tie-break). Each adjacent pair in a list is an edge. Time O(V+E).
import java.util.*;

public class Solution {
    static List<Integer> interleave(int[][] lists) {
        List<Integer> order = new ArrayList<>();
        Set<Integer> seen = new HashSet<>();
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Set<Long> edges = new HashSet<>();
        Map<Integer, Integer> indeg = new HashMap<>();

        for (int[] lst : lists) {
            for (int x : lst)
                if (seen.add(x)) { order.add(x); adj.put(x, new ArrayList<>()); indeg.put(x, 0); }
            for (int i = 0; i + 1 < lst.length; i++) {
                int a = lst[i], b = lst[i + 1];
                long key = ((long) a << 32) | (b & 0xffffffffL);
                if (edges.add(key)) { adj.get(a).add(b); indeg.put(b, indeg.get(b) + 1); }
            }
        }
        Queue<Integer> q = new ArrayDeque<>();
        for (int x : order) if (indeg.get(x) == 0) q.add(x);
        List<Integer> res = new ArrayList<>();
        while (!q.isEmpty()) {
            int u = q.poll();
            res.add(u);
            for (int v : adj.get(u))
                if (indeg.merge(v, -1, Integer::sum) == 0) q.add(v);
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] lists = {{1,7,3},{2,1,6,7,9},{3,9,5}};
        System.out.println(interleave(lists)); // [2, 1, 6, 7, 3, 9, 5]
    }
}
