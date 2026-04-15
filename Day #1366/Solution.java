// Bipartite check via BFS 2-coloring. Time O(V+E), Space O(V).
import java.util.*;

public class Solution {
    static boolean isBipartite(int n, List<List<Integer>> adj) {
        int[] color = new int[n];
        Arrays.fill(color, -1);
        for (int s = 0; s < n; s++) {
            if (color[s] != -1) continue;
            Deque<Integer> q = new ArrayDeque<>();
            q.add(s); color[s] = 0;
            while (!q.isEmpty()) {
                int u = q.poll();
                for (int v : adj.get(u)) {
                    if (color[v] == -1) { color[v] = color[u] ^ 1; q.add(v); }
                    else if (color[v] == color[u]) return false;
                }
            }
        }
        return true;
    }

    static List<List<Integer>> build(int n, int[][] edges) {
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        return adj;
    }

    public static void main(String[] args) {
        System.out.println(isBipartite(4, build(4, new int[][]{{0,1},{1,2},{2,3},{3,0}})));
        System.out.println(isBipartite(3, build(3, new int[][]{{0,1},{1,2},{2,0}})));
    }
}
