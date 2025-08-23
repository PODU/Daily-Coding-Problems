// Day 160: Weighted tree diameter. One DFS; each node returns its longest
// downward branch, while we combine the top two branches. Time O(V+E).
import java.util.*;

public class Solution {
    static Map<String, List<Object[]>> adj = new HashMap<>();
    static long best = 0;

    static void addEdge(String u, String v, int w) {
        adj.computeIfAbsent(u, k -> new ArrayList<>()).add(new Object[]{v, w});
        adj.computeIfAbsent(v, k -> new ArrayList<>()).add(new Object[]{u, w});
    }

    static long dfs(String node, String parent) {
        long top1 = 0, top2 = 0;
        for (Object[] e : adj.getOrDefault(node, Collections.emptyList())) {
            String nb = (String) e[0];
            int w = (int) e[1];
            if (nb.equals(parent)) continue;
            long d = w + dfs(nb, node);
            if (d > top1) { top2 = top1; top1 = d; }
            else if (d > top2) { top2 = d; }
        }
        best = Math.max(best, top1 + top2);
        return top1;
    }

    public static void main(String[] args) {
        addEdge("a", "b", 3); addEdge("a", "c", 5); addEdge("a", "d", 8);
        addEdge("d", "e", 2); addEdge("d", "f", 4);
        addEdge("e", "g", 1); addEdge("e", "h", 1);
        dfs("a", "");
        System.out.println(best); // 17
    }
}
