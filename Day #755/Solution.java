// Day 755: Largest value path in a directed graph.
// Topological DP: dp[u][c] = max count of letter c on a path starting at u.
// Cycle -> value is infinite -> null. Time: O((n+m)*26), Space: O(n*26).
import java.util.*;

public class Solution {
    // returns null when the value is infinite (cycle present)
    static Integer largestPathValue(String letters, int[][] edges) {
        int n = letters.length();
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[] indeg = new int[n];
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); indeg[e[1]]++; }

        Deque<Integer> q = new ArrayDeque<>();
        for (int i = 0; i < n; i++) if (indeg[i] == 0) q.add(i);
        List<Integer> topo = new ArrayList<>();
        while (!q.isEmpty()) {
            int u = q.poll();
            topo.add(u);
            for (int v : adj.get(u)) if (--indeg[v] == 0) q.add(v);
        }
        if (topo.size() < n) return null;   // cycle

        int[][] dp = new int[n][26];
        for (int i = 0; i < n; i++) dp[i][letters.charAt(i) - 'A'] = 1;

        int best = 0;
        for (int i = topo.size() - 1; i >= 0; i--) {
            int u = topo.get(i);
            int uc = letters.charAt(u) - 'A';
            for (int v : adj.get(u))
                for (int c = 0; c < 26; c++) {
                    int add = dp[v][c] + (c == uc ? 1 : 0);
                    if (add > dp[u][c]) dp[u][c] = add;
                }
            for (int c = 0; c < 26; c++) best = Math.max(best, dp[u][c]);
        }
        return best;
    }

    public static void main(String[] args) {
        String letters = "ABACA";
        int[][] edges = {{0,1},{0,2},{2,3},{3,4}};
        Integer res = largestPathValue(letters, edges);
        System.out.println(res == null ? "null" : res);  // 3
    }
}
