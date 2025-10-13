// Day 423: Transitive closure via DFS from each vertex. O(V*(V+E)) time, O(V^2) space.
// M[i][j] = 1 iff j is reachable from i (each vertex reaches itself).
import java.util.*;

public class Solution {
    static int[][] M;
    static List<List<Integer>> g;
    static int n;

    static void dfs(int src, int u) {
        M[src][u] = 1;
        for (int v : g.get(u))
            if (M[src][v] == 0) dfs(src, v);
    }

    public static void main(String[] args) {
        int[][] graph = {{0, 1, 3}, {1, 2}, {2}, {3}};
        n = graph.length;
        g = new ArrayList<>();
        for (int[] row : graph) {
            List<Integer> l = new ArrayList<>();
            for (int x : row) l.add(x);
            g.add(l);
        }
        M = new int[n][n];
        for (int i = 0; i < n; i++) dfs(i, i);

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append("[");
            for (int j = 0; j < n; j++) {
                sb.append(M[i][j]);
                if (j + 1 < n) sb.append(", ");
            }
            sb.append("]\n");
        }
        System.out.print(sb);
    }
}
