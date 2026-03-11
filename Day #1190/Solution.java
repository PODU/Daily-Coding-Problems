// Dijkstra from node 0 over nodes 0..N (undirected); answer = max finite shortest-path distance.
// Time: O(E log V), Space: O(V + E).
import java.util.*;

public class Solution {
    static int networkDelay(int N, int[][] edges) {
        int V = N + 1;
        List<int[]>[] adj = new List[V];
        for (int i = 0; i < V; i++) adj[i] = new ArrayList<>();
        for (int[] e : edges) {
            adj[e[0]].add(new int[]{e[1], e[2]});
            adj[e[1]].add(new int[]{e[0], e[2]});
        }
        long[] dist = new long[V];
        Arrays.fill(dist, Long.MAX_VALUE);
        dist[0] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, 0});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0]; int u = (int) top[1];
            if (d > dist[u]) continue;
            for (int[] nx : adj[u]) {
                int v = nx[0], w = nx[1];
                if (dist[u] + w < dist[v]) {
                    dist[v] = dist[u] + w;
                    pq.add(new long[]{dist[v], v});
                }
            }
        }
        long ans = 0;
        for (long d : dist) if (d != Long.MAX_VALUE) ans = Math.max(ans, d);
        return (int) ans;
    }

    public static void main(String[] args) {
        int N = 5;
        int[][] edges = {{0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}};
        System.out.println(networkDelay(N, edges));
    }
}
