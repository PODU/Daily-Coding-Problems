// Max edges removed so every component has even node count. DFS subtree sizes;
// answer = count of non-root nodes with even subtree size. Time O(n), Space O(n).
// Note: README narrates one valid single cut (3,4), but the MAXIMUM is 2:
// cut (1,3) and (3,4) -> {1,2},{3,5},{4,6,7,8}, all even.
import java.util.*;

public class Solution {
    static List<List<Integer>> adj;
    static int answer = 0;

    static int dfs(int u, int parent, int root) {
        int s = 1;
        for (int v : adj.get(u))
            if (v != parent) s += dfs(v, u, root);
        if (u != root && s % 2 == 0) answer++;
        return s;
    }

    public static void main(String[] args) {
        int n = 8;
        adj = new ArrayList<>();
        for (int i = 0; i <= n; i++) adj.add(new ArrayList<>());
        int[][] edges = {{1, 2}, {1, 3}, {3, 4}, {3, 5}, {4, 6}, {4, 7}, {4, 8}};
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            adj.get(e[1]).add(e[0]);
        }
        dfs(1, 0, 1);
        System.out.println(answer);
    }
}
