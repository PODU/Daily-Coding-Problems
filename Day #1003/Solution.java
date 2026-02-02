// Day 1003: Transitive closure of a graph (adjacency list -> reachability matrix).
// DFS from each vertex marking everything reachable. O(V*(V+E)) time, O(V^2) space.
import java.util.*;

public class Solution {
    static int[][] graph;
    static int[][] M;

    static void dfs(int start, int u) {
        for (int v : graph[u]) {
            if (M[start][v] == 0) {
                M[start][v] = 1;
                dfs(start, v);
            }
        }
    }

    public static void main(String[] args) {
        graph = new int[][]{{0, 1, 3}, {1, 2}, {2}, {3}};
        int n = graph.length;
        M = new int[n][n];
        for (int s = 0; s < n; s++) { M[s][s] = 1; dfs(s, s); }
        for (int[] row : M) System.out.println(Arrays.toString(row));
    }
}
