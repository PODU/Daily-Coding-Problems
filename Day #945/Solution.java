// Day 945: Longest path (diameter) in a weighted tree.
// DFS: at each node combine its two deepest downward branches. Time O(V+E), Space O(V).
import java.util.*;

public class Solution {
    static Map<String, List<Object[]>> adj = new HashMap<>();
    static int best = 0;

    static void addEdge(String u, String v, int w) {
        adj.computeIfAbsent(u, k -> new ArrayList<>()).add(new Object[]{v, w});
        adj.computeIfAbsent(v, k -> new ArrayList<>()).add(new Object[]{u, w});
    }

    // Returns the longest downward path length from node (excluding edge above it).
    static int dfs(String node, String parent) {
        int max1 = 0, max2 = 0;
        for (Object[] e : adj.getOrDefault(node, Collections.emptyList())) {
            String nb = (String) e[0];
            int w = (int) e[1];
            if (nb.equals(parent)) continue;
            int d = dfs(nb, node) + w;
            if (d > max1) { max2 = max1; max1 = d; }
            else if (d > max2) { max2 = d; }
        }
        best = Math.max(best, max1 + max2);
        return max1;
    }

    public static void main(String[] args) {
        addEdge("a","b",3); addEdge("a","c",5); addEdge("a","d",8);
        addEdge("d","e",2); addEdge("d","f",4);
        addEdge("e","g",1); addEdge("e","h",1);
        dfs("a", "");
        System.out.println(best); // 17 (path c -> a -> d -> f)
    }
}
