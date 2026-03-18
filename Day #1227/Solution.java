// Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
// skipping conflicts. Time O(k^n) worst case, Space O(n).
public class Solution {
    static boolean safe(int v, int[][] g, int[] color, int c) {
        for (int i = 0; i < g.length; i++)
            if (g[v][i] == 1 && color[i] == c) return false;
        return true;
    }

    static boolean colorize(int v, int[][] g, int k, int[] color) {
        if (v == g.length) return true;
        for (int c = 1; c <= k; c++) {
            if (safe(v, g, color, c)) {
                color[v] = c;
                if (colorize(v + 1, g, k, color)) return true;
                color[v] = 0;
            }
        }
        return false;
    }

    static boolean isKColorable(int[][] g, int k) {
        return colorize(0, g, k, new int[g.length]);
    }

    public static void main(String[] args) {
        int[][] g = {{0,1,1},{1,0,1},{1,1,0}};
        System.out.println(isKColorable(g, 2));
        System.out.println(isKColorable(g, 3));
    }
}
