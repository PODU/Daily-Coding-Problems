// Day 270: Broadcast time = max shortest-path distance from node 0 (Dijkstra).
// Min-heap Dijkstra over undirected weighted graph; answer = max finite dist. Time O(E log V), Space O(V+E).
import java.util.*;

public class Solution {
    static int networkDelay(int n, int[][] edges) {
        List<int[]>[] adj = new List[n + 1];
        for (int i = 0; i <= n; i++) adj[i] = new ArrayList<>();
        for (int[] e : edges) {
            adj[e[0]].add(new int[]{e[1], e[2]});
            adj[e[1]].add(new int[]{e[0], e[2]});
        }
        long[] dist = new long[n + 1];
        Arrays.fill(dist, Long.MAX_VALUE);
        dist[0] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, 0});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0]; int u = (int) top[1];
            if (d > dist[u]) continue;
            for (int[] nb : adj[u]) {
                int v = nb[0], w = nb[1];
                if (d + w < dist[v]) { dist[v] = d + w; pq.add(new long[]{dist[v], v}); }
            }
        }
        long ans = 0;
        for (long d : dist) ans = Math.max(ans, d);
        return (int) ans;
    }

    public static void main(String[] args) {
        int[][] edges = {
            {0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}
        };
        System.out.println(networkDelay(5, edges));
    }
}
