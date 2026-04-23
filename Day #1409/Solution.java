// Transitive closure: each input row is [node, neighbors...]. DFS from every node
// marks all reachable vertices (incl. itself). Time O(V*(V+E)), Space O(V^2).
import java.util.*;

public class Solution {
    static void dfs(int u, List<List<Integer>> adj, int[] row) {
        row[u] = 1;
        for (int v : adj.get(u)) if (row[v] == 0) dfs(v, adj, row);
    }

    static int[][] transitiveClosure(int[][] graph) {
        int n = graph.length;
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] r : graph) {
            int node = r[0];
            for (int i = 1; i < r.length; i++) adj.get(node).add(r[i]);
        }
        int[][] M = new int[n][n];
        for (int i = 0; i < n; i++) dfs(i, adj, M[i]);
        return M;
    }

    public static void main(String[] args) {
        int[][] graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
        for (int[] row : transitiveClosure(graph))
            System.out.println(Arrays.toString(row));
    }
}
