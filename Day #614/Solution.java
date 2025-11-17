// Day 614: Time for a message from node 0 to reach all nodes = max shortest-path distance.
// Approach: Dijkstra from node 0, return the largest distance. Time O(E log V), Space O(V+E).
import java.util.*;

public class Solution {
    static long broadcastTime(int n, int[][] edges) {
        List<long[]>[] adj = new List[n + 1];
        for (int i = 0; i <= n; i++) adj[i] = new ArrayList<>();
        for (int[] e : edges) adj[e[0]].add(new long[]{e[1], e[2]});

        long[] dist = new long[n + 1];
        Arrays.fill(dist, Long.MAX_VALUE);
        dist[0] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, 0});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0];
            int u = (int) top[1];
            if (d > dist[u]) continue;
            for (long[] e : adj[u]) {
                int v = (int) e[0];
                long w = e[1];
                if (d + w < dist[v]) { dist[v] = d + w; pq.add(new long[]{dist[v], v}); }
            }
        }
        long ans = 0;
        for (int i = 0; i <= n; i++) ans = Math.max(ans, dist[i]);
        return ans;
    }

    public static void main(String[] args) {
        int N = 5;
        int[][] edges = {
            {0,1,5}, {0,2,3}, {0,5,4}, {1,3,8}, {2,3,1}, {3,5,10}, {3,4,5}
        };
        System.out.println(broadcastTime(N, edges)); // 9
    }
}
