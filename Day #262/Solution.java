// Day 262: Find all bridges in a connected undirected graph.
// Approach: Tarjan's bridge-finding algorithm using DFS with disc/low timestamps.
// An edge (u,v) is a bridge iff low[v] > disc[u]. Time O(V+E), Space O(V+E).

import java.util.*;

public class Solution {
    private int n, timer = 0;
    private List<List<Integer>> adj;
    private int[] disc, low;
    private List<int[]> bridges = new ArrayList<>();

    public Solution(int n) {
        this.n = n;
        adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        disc = new int[n];
        low = new int[n];
    }

    public void addEdge(int u, int v) { adj.get(u).add(v); adj.get(v).add(u); }

    private void dfs(int u, int parent) {
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

    public List<int[]> findBridges() {
        for (int i = 0; i < n; i++) if (disc[i] == 0) dfs(i, -1);
        bridges.sort((a, b) -> a[0] != b[0] ? a[0] - b[0] : a[1] - b[1]);
        return bridges;
    }

    public static void main(String[] args) {
        Solution g = new Solution(5);
        g.addEdge(0, 1); g.addEdge(1, 2); g.addEdge(2, 0);
        g.addEdge(1, 3); g.addEdge(3, 4);
        List<int[]> bridges = g.findBridges();
        StringBuilder sb = new StringBuilder("Bridges: [");
        for (int i = 0; i < bridges.size(); i++) {
            sb.append("(").append(bridges.get(i)[0]).append(", ").append(bridges.get(i)[1]).append(")");
            if (i + 1 < bridges.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
