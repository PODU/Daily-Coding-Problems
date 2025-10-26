// Day 492: Graph m-colorability via backtracking.
// Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
// Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.
public class Solution {
    static boolean isSafe(int v, int[][] graph, int[] colors, int c) {
        for (int u = 0; u < graph.length; u++)
            if (graph[v][u] == 1 && colors[u] == c) return false;
        return true;
    }

    static boolean solve(int v, int[][] graph, int k, int[] colors) {
        if (v == graph.length) return true;
        for (int c = 1; c <= k; c++) {
            if (isSafe(v, graph, colors, c)) {
                colors[v] = c;
                if (solve(v + 1, graph, k, colors)) return true;
                colors[v] = 0;
            }
        }
        return false;
    }

    static boolean canColor(int[][] graph, int k) {
        return solve(0, graph, k, new int[graph.length]);
    }

    public static void main(String[] args) {
        // Triangle K3: every pair adjacent.
        int[][] graph = {
            {0, 1, 1},
            {1, 0, 1},
            {1, 1, 0}};
        System.out.println("k=2 colorable: " + canColor(graph, 2));
        System.out.println("k=3 colorable: " + canColor(graph, 3));
    }
}
