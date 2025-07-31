// Day 56: Graph k-colorability via backtracking.
// Time: O(k^V) worst case, Space: O(V).
public class Solution {
    static boolean canColor(int[][] g, int k, int[] color, int v) {
        int n = g.length;
        if (v == n) return true;
        for (int c = 1; c <= k; c++) {
            boolean ok = true;
            for (int u = 0; u < n; u++)
                if (g[v][u] == 1 && color[u] == c) { ok = false; break; }
            if (!ok) continue;
            color[v] = c;
            if (canColor(g, k, color, v + 1)) return true;
            color[v] = 0;
        }
        return false;
    }

    static boolean kColorable(int[][] g, int k) {
        return canColor(g, k, new int[g.length], 0);
    }

    public static void main(String[] args) {
        // Triangle graph: needs 3 colors.
        int[][] g = {
            {0, 1, 1},
            {1, 0, 1},
            {1, 1, 0}
        };
        System.out.println(kColorable(g, 2)); // false
        System.out.println(kColorable(g, 3)); // true
    }
}
