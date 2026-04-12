// Weighted tree diameter: DFS, at each node track two largest downward child path sums;
// diameter = max over nodes of (sum of two largest). Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static List<int[]>[] adj;
    static int best = 0;

    @SuppressWarnings("unchecked")
    static void add(int a, int b, int w) {
        adj[a].add(new int[]{b, w});
        adj[b].add(new int[]{a, w});
    }

    static int dfs(int u, int parent) {
        int max1 = 0, max2 = 0;
        for (int[] e : adj[u]) {
            int v = e[0], w = e[1];
            if (v == parent) continue;
            int down = dfs(v, u) + w;
            if (down > max1) { max2 = max1; max1 = down; }
            else if (down > max2) { max2 = down; }
        }
        best = Math.max(best, max1 + max2);
        return max1;
    }

    @SuppressWarnings("unchecked")
    public static void main(String[] args) {
        int n = 8; // a..h -> 0..7
        adj = new List[n];
        for (int i = 0; i < n; i++) adj[i] = new ArrayList<>();
        add(0, 1, 3); // a-b
        add(0, 2, 5); // a-c
        add(0, 3, 8); // a-d
        add(3, 4, 2); // d-e
        add(3, 5, 4); // d-f
        add(4, 6, 1); // e-g
        add(4, 7, 1); // e-h
        dfs(0, -1);
        System.out.println(best);
    }
}
