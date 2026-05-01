// Day 1454: A graph is minimally-connected iff it is a tree: connected AND has
// no cycle (exactly n-1 edges). DFS from node 0. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static boolean isTree(int n, int[][] edges) {
        if (n == 0) return true;
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < n; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        boolean[] visited = new boolean[n];
        Deque<int[]> st = new ArrayDeque<>();
        st.push(new int[]{0, -1}); visited[0] = true; int seen = 1;
        while (!st.isEmpty()) {
            int[] cur = st.pop();
            int u = cur[0], p = cur[1];
            for (int w : adj.get(u)) {
                if (!visited[w]) { visited[w] = true; seen++; st.push(new int[]{w, u}); }
                else if (w != p) return false; // back-edge -> cycle
            }
        }
        return seen == n;
    }

    public static void main(String[] args) {
        int[][] tree = {{0,1},{1,2},{1,3}};
        int[][] cyclic = {{0,1},{1,2},{2,0},{2,3}};
        System.out.println(isTree(4, tree) ? "True" : "False");   // True
        System.out.println(isTree(4, cyclic) ? "True" : "False"); // False
    }
}
