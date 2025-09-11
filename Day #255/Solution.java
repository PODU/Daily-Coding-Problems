// Transitive closure: DFS from each vertex marking reachable nodes (incl self).
// Time O(V*(V+E)), Space O(V^2) for the matrix.
public class Solution {
    static void dfs(int u, int[][] g, int[] row) {
        row[u] = 1;
        for (int v : g[u]) if (row[v] == 0) dfs(v, g, row);
    }

    public static void main(String[] args) {
        int[][] graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
        int n = graph.length;
        int[][] M = new int[n][n];
        for (int i = 0; i < n; i++) dfs(i, graph, M[i]);

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append("[");
            for (int j = 0; j < n; j++) {
                sb.append(M[i][j]);
                if (j + 1 < n) sb.append(", ");
            }
            sb.append("]");
            System.out.println(sb.toString());
            sb.setLength(0);
        }
    }
}
