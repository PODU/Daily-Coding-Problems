// Remove max edges so each resulting subtree has an even node count.
// DFS subtree sizes; count non-root nodes whose subtree size is even (each = one removable edge above it). O(n) time/space.
import java.util.*;

public class Solution {
    static List<List<Integer>> adj;
    static int removable = 0;

    static int dfs(int u, int parent) {
        int size = 1;
        for (int v : adj.get(u)) if (v != parent) size += dfs(v, u);
        if (parent != -1 && size % 2 == 0) removable++;
        return size;
    }

    public static void main(String[] args) {
        int n = 8;
        adj = new ArrayList<>();
        for (int i = 0; i <= n; i++) adj.add(new ArrayList<>());
        int[][] edges = {{1,2},{1,3},{3,4},{3,5},{4,6},{4,7},{4,8}};
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }
        dfs(1, -1);
        System.out.println(removable);
    }
}
