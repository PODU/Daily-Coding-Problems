// Day 1682: Graph k-colorability via backtracking with pruning.
// Time O(k^V) worst case, Space O(V).
public class Solution {
    static int[][] adj;
    static int[] color;
    static int n, k;

    static boolean solve(int v) {
        if (v == n) return true;
        for (int c = 0; c < k; c++) {
            boolean ok = true;
            for (int u = 0; u < n; u++)
                if (adj[v][u] == 1 && color[u] == c) { ok = false; break; }
            if (ok) {
                color[v] = c;
                if (solve(v + 1)) return true;
                color[v] = -1;
            }
        }
        return false;
    }

    static boolean canColor(int[][] a, int colors) {
        adj = a; n = a.length; k = colors;
        color = new int[n];
        java.util.Arrays.fill(color, -1);
        return solve(0);
    }

    public static void main(String[] args) {
        int[][] tri = {{0,1,1},{1,0,1},{1,1,0}};
        System.out.println(canColor(tri, 2) ? "True" : "False"); // False
        System.out.println(canColor(tri, 3) ? "True" : "False"); // True
    }
}
