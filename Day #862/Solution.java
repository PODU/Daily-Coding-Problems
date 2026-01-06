// Day 862: Find all bridges in a connected undirected graph.
// Approach: Tarjan's DFS using discovery times and low-link values.
// Time: O(V + E), Space: O(V + E).
import java.util.*;

public class Solution {
    static List<List<Integer>> adj;
    static int[] disc, low;
    static int timer = 0;
    static List<int[]> bridges = new ArrayList<>();

    static void dfs(int u, int parent) {
        disc[u] = low[u] = ++timer;
        for (int v : adj.get(u)) {
            if (v == parent) continue;
            if (disc[v] != 0) low[u] = Math.min(low[u], disc[v]);
            else {
                dfs(v, u);
                low[u] = Math.min(low[u], low[v]);
                if (low[v] > disc[u]) bridges.add(new int[]{Math.min(u, v), Math.max(u, v)});
            }
        }
    }

    public static void main(String[] args) {
        int n = 5;
        adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[][] edges = {{0,1},{1,2},{2,0},{1,3},{3,4}};
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        disc = new int[n]; low = new int[n];
        for (int i = 0; i < n; i++) if (disc[i] == 0) dfs(i, -1);

        bridges.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        StringBuilder sb = new StringBuilder("Bridges:");
        for (int[] b : bridges) sb.append(" (").append(b[0]).append(", ").append(b[1]).append(")");
        System.out.println(sb);   // (1, 3) (3, 4)
    }
}
