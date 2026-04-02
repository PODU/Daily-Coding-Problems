// Day 1286: Find all bridges in an undirected graph (Tarjan's low-link DFS).
// Time O(V + E), Space O(V + E).
import java.util.*;

public class Solution {
    static List<List<Integer>> adj;
    static int[] disc, low;
    static int timer = 0;
    static List<int[]> bridges = new ArrayList<>();

    static void dfs(int u, int parent) {
        disc[u] = low[u] = ++timer;
        boolean skippedParent = false;
        for (int v : adj.get(u)) {
            if (v == parent && !skippedParent) { skippedParent = true; continue; }
            if (disc[v] == 0) {
                dfs(v, u);
                low[u] = Math.min(low[u], low[v]);
                if (low[v] > disc[u]) bridges.add(new int[]{Math.min(u, v), Math.max(u, v)});
            } else {
                low[u] = Math.min(low[u], disc[v]);
            }
        }
    }

    public static void main(String[] args) {
        int n = 5;
        int[][] edges = {{0, 1}, {1, 2}, {2, 0}, {1, 3}, {3, 4}};
        adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        disc = new int[n];
        low = new int[n];
        for (int i = 0; i < n; i++) if (disc[i] == 0) dfs(i, -1);
        bridges.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        for (int[] b : bridges) System.out.println(b[0] + " - " + b[1]);
    }
}
