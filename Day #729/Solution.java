// Day 729: Interleave ranked preference lists into one consistent playlist.
// Approach: Build precedence DAG (consecutive pairs), Kahn topological sort (FIFO,
// first-appearance tie-break). Time: O(V + E), Space: O(V + E).
import java.util.*;

public class Solution {
    static List<Integer> interleave(int[][] lists) {
        Map<Integer, List<Integer>> adj = new HashMap<>();
        Map<Integer, Integer> indeg = new HashMap<>();
        Set<Long> edges = new HashSet<>();
        List<Integer> order = new ArrayList<>();
        for (int[] lst : lists) {
            for (int x : lst)
                if (!indeg.containsKey(x)) { indeg.put(x, 0); adj.put(x, new ArrayList<>()); order.add(x); }
            for (int i = 0; i + 1 < lst.length; i++) {
                long key = (long) lst[i] * 1000000L + lst[i + 1];
                if (edges.add(key)) { adj.get(lst[i]).add(lst[i + 1]); indeg.merge(lst[i + 1], 1, Integer::sum); }
            }
        }
        Deque<Integer> q = new ArrayDeque<>();
        for (int x : order) if (indeg.get(x) == 0) q.add(x);
        List<Integer> res = new ArrayList<>();
        while (!q.isEmpty()) {
            int u = q.poll();
            res.add(u);
            for (int v : adj.get(u)) if (indeg.merge(v, -1, Integer::sum) == 0) q.add(v);
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] lists = {{1, 7, 3}, {2, 1, 6, 7, 9}, {3, 9, 5}};
        System.out.println(interleave(lists));
    }
}
