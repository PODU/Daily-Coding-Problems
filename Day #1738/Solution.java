// Topological-order DP over DAG: dp[node][c]=max count of letter c on path from node.
// Kahn's algorithm; cycle => null. Time O((n+m)*26), Space O(n*26).
import java.util.*;

public class Solution {
    static String largestPathValue(String s, int[][] edges) {
        int n = s.length();
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[n];
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); indeg[e[1]]++; }
        int[][] dp = new int[n][26];
        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < n; i++) if (indeg[i] == 0) q.add(i);
        int seen = 0, ans = 0;
        while (!q.isEmpty()) {
            int u = q.poll(); seen++;
            int cu = s.charAt(u) - 'A';
            dp[u][cu]++;
            ans = Math.max(ans, dp[u][cu]);
            for (int v : adj.get(u)) {
                for (int c = 0; c < 26; c++) dp[v][c] = Math.max(dp[v][c], dp[u][c]);
                if (--indeg[v] == 0) q.add(v);
            }
        }
        return seen < n ? "null" : String.valueOf(ans);
    }

    public static void main(String[] args) {
        System.out.println(largestPathValue("ABACA", new int[][]{{0,1},{0,2},{2,3},{3,4}}));
        System.out.println(largestPathValue("A", new int[][]{{0,0}}));
    }
}
