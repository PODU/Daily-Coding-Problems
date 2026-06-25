// Transitive closure via DFS from each vertex. Time O(V*(V+E)), Space O(V^2).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] graph = {{0,1,3},{1,2},{2},{3}};
        int n = graph.length;
        int[][] M = new int[n][n];
        for (int s = 0; s < n; s++) {
            boolean[] vis = new boolean[n];
            Deque<Integer> st = new ArrayDeque<>();
            st.push(s);
            while (!st.isEmpty()) {
                int u = st.pop();
                if (vis[u]) continue;
                vis[u] = true; M[s][u] = 1;
                for (int v : graph[u]) if (!vis[v]) st.push(v);
            }
        }
        for (int i = 0; i < n; i++) {
            StringBuilder sb = new StringBuilder("[");
            for (int j = 0; j < n; j++) {
                sb.append(M[i][j]);
                if (j + 1 < n) sb.append(", ");
            }
            sb.append("]");
            System.out.println(sb.toString());
        }
    }
}
