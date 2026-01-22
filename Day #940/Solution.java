// Day 940: Time for a message from node 0 to reach all = max shortest-path distance (Dijkstra).
// Time O(E log V), Space O(V + E). Returns -1 if some node is unreachable.
import java.util.*;

public class Solution {
    static int networkDelay(int n, int[][] edges, int src) {
        List<int[]>[] adj = new List[n + 1];
        for (int i = 0; i <= n; i++) adj[i] = new ArrayList<>();
        for (int[] e : edges) adj[e[0]].add(new int[]{e[1], e[2]});

        int[] dist = new int[n + 1];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
        pq.add(new int[]{0, src});
        while (!pq.isEmpty()) {
            int[] top = pq.poll();
            int d = top[0], u = top[1];
            if (d > dist[u]) continue;
            for (int[] nb : adj[u]) {
                int v = nb[0], w = nb[1];
                if (d + w < dist[v]) { dist[v] = d + w; pq.add(new int[]{dist[v], v}); }
            }
        }
        int ans = 0;
        for (int i = 0; i <= n; i++) {
            if (dist[i] == Integer.MAX_VALUE) return -1;
            ans = Math.max(ans, dist[i]);
        }
        return ans;
    }

    public static void main(String[] args) {
        int n = 5;
        int[][] edges = {
            {0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}};
        System.out.println(networkDelay(n, edges, 0)); // 9
    }
}
