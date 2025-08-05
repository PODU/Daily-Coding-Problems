// Largest path value in directed graph: topo sort (Kahn) + DP dp[node][26]. Cycle -> null. Time O((n+m)*26), Space O(n*26).
import java.util.*;

public class Solution {
    // Graph "A" with edge (0,0) returns null (self-loop is a cycle).
    // Returns null if the graph is cyclic, else the largest path value.
    static Integer largestPathValue(String colors, int[][] edges) {
        int n = colors.length();
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[n];
        for (int[] e : edges) {
            adj.get(e[0]).add(e[1]);
            indeg[e[1]]++;
        }
        int[][] dp = new int[n][26];
        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < n; i++) if (indeg[i] == 0) q.add(i);
        int seen = 0, ans = 0;
        while (!q.isEmpty()) {
            int u = q.poll();
            seen++;
            int cu = colors.charAt(u) - 'A';
            dp[u][cu]++;
            ans = Math.max(ans, dp[u][cu]);
            for (int v : adj.get(u)) {
                for (int c = 0; c < 26; c++)
                    dp[v][c] = Math.max(dp[v][c], dp[u][c]);
                if (--indeg[v] == 0) q.add(v);
            }
        }
        if (seen < n) return null; // cycle
        return ans;
    }

    public static void main(String[] args) {
        String colors = "ABACA";
        int[][] edges = {{0,1},{0,2},{2,3},{3,4}};
        Integer result = largestPathValue(colors, edges);
        System.out.println(result == null ? "null" : result);
    }
}
