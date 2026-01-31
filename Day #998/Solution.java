// Day 998: Graph k-colorability (adjacency matrix).
// Backtracking: try each color per vertex, skipping colors used by neighbors.
// O(k^V) worst case, O(V) extra space.
public class Solution {
    static int[][] g;
    static int[] colors;
    static int n, K;

    static boolean ok(int v, int c) {
        for (int u = 0; u < n; u++)
            if (g[v][u] == 1 && colors[u] == c) return false;
        return true;
    }

    static boolean solve(int v) {
        if (v == n) return true;
        for (int c = 1; c <= K; c++) {
            if (ok(v, c)) {
                colors[v] = c;
                if (solve(v + 1)) return true;
                colors[v] = 0;
            }
        }
        return false;
    }

    static boolean canColor(int[][] graph, int k) {
        g = graph; n = graph.length; K = k;
        colors = new int[n];
        return solve(0);
    }

    public static void main(String[] args) {
        int[][] triangle = {{0, 1, 1}, {1, 0, 1}, {1, 1, 0}};
        System.out.println(canColor(triangle, 2)); // false
        System.out.println(canColor(triangle, 3)); // true
    }
}
