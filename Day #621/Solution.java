// Tree diameter with edge weights: DFS returning max downward path; global best
// = sum of two largest child paths. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static List<long[]>[] adj;
    static long best = 0;

    @SuppressWarnings("unchecked")
    static void build(int n) {
        adj = new List[n];
        for (int i = 0; i < n; i++) adj[i] = new ArrayList<>();
    }

    static void add(int u, int v, long w) {
        adj[u].add(new long[]{v, w});
        adj[v].add(new long[]{u, w});
    }

    static long dfs(int u, int parent) {
        long max1 = 0, max2 = 0;
        for (long[] e : adj[u]) {
            if (e[0] == parent) continue;
            long path = dfs((int) e[0], u) + e[1];
            if (path > max1) { max2 = max1; max1 = path; }
            else if (path > max2) { max2 = path; }
        }
        best = Math.max(best, max1 + max2);
        return max1;
    }

    public static void main(String[] args) {
        build(8); // a0 b1 c2 d3 e4 f5 g6 h7
        add(0,1,3); add(0,2,5); add(0,3,8);
        add(3,4,2); add(3,5,4);
        add(4,6,1); add(4,7,1);
        dfs(0, -1);
        System.out.println(best);
    }
}
