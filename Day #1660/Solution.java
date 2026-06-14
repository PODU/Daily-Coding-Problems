// DFS subtree sizes; count non-root subtrees with even size = max edges removable.
// Time O(n), Space O(n).
import java.util.*;
public class Solution {
    static List<List<Integer>> g;
    static int ans = 0;
    static int dfs(int u, int p) {
        int sz = 1;
        for (int v : g.get(u)) if (v != p) sz += dfs(v, u);
        if (p != -1 && sz % 2 == 0) ans++;
        return sz;
    }
    public static void main(String[] args) {
        int n = 8;
        g = new ArrayList<>();
        for (int i = 0; i <= n; i++) g.add(new ArrayList<>());
        int[][] edges = {{1,2},{1,3},{3,4},{3,5},{4,6},{4,7},{4,8}};
        for (int[] e : edges) { g.get(e[0]).add(e[1]); g.get(e[1]).add(e[0]); }
        dfs(1, -1);
        System.out.println(ans);
    }
}
