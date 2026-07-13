// Day 1805: Find all bridges via Tarjan's algorithm (disc[]/low[], edge is bridge if low[v] > disc[u]).
// Parallel edges handled by skipping the parent edge once via edge id. O(V+E).
import java.util.*;

public class Solution {
    static List<int[]>[] adj;
    static int[] disc, low;
    static int timer = 0;
    static List<int[]> result = new ArrayList<>();

    static void dfs(int u, int parentEdge) {
        disc[u] = low[u] = timer++;
        for (int[] e : adj[u]) {
            int v = e[0], id = e[1];
            if (id == parentEdge) continue;        // skip the single parent edge once
            if (disc[v] == -1) {
                dfs(v, id);
                low[u] = Math.min(low[u], low[v]);
                if (low[v] > disc[u]) result.add(new int[]{Math.min(u,v), Math.max(u,v)});
            } else {
                low[u] = Math.min(low[u], disc[v]);
            }
        }
    }

    @SuppressWarnings("unchecked")
    public static void main(String[] args) {
        int n = 5;
        adj = new List[n];
        for (int i = 0; i < n; i++) adj[i] = new ArrayList<>();
        int[][] edges = {{0,1},{1,2},{2,0},{1,3},{3,4}};
        for (int i = 0; i < edges.length; i++) {
            adj[edges[i][0]].add(new int[]{edges[i][1], i});
            adj[edges[i][1]].add(new int[]{edges[i][0], i});
        }
        disc = new int[n]; low = new int[n];
        Arrays.fill(disc, -1); Arrays.fill(low, -1);
        for (int i = 0; i < n; i++) if (disc[i] == -1) dfs(i, -1);
        result.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        for (int[] e : result) System.out.println(e[0] + " - " + e[1]);
        // expected: 1 - 3 and 3 - 4
    }
}
