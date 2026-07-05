// Uphill-then-downhill shortest cyclic route from home (node 0): Dijkstra over
// states (node, phase). UP edges require rising elevation, DOWN edges require
// falling; a free phase switch (the peak) is allowed at non-home nodes.
// Time: O(E log V), Space: O(V+E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] elev = {5, 25, 15, 20, 10};
        int[][] paths = {
            {0,1,10},{0,2,8},{0,3,15},{1,3,12},
            {2,4,10},{3,4,5},{3,0,17},{4,0,10}
        };
        int n = elev.length;
        List<int[]>[] adj = new List[n];
        for (int i = 0; i < n; i++) adj[i] = new ArrayList<>();
        for (int[] p : paths) adj[p[0]].add(new int[]{p[1], p[2]});

        long INF = Long.MAX_VALUE;
        long[] dist = new long[2 * n];
        Arrays.fill(dist, INF);
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        dist[0] = 0;
        pq.add(new long[]{0, 0});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0];
            int s = (int) top[1];
            if (d > dist[s]) continue;
            int node = s / 2, phase = s % 2;
            if (phase == 0 && node != 0) {
                int ns = node * 2 + 1;
                if (d < dist[ns]) { dist[ns] = d; pq.add(new long[]{d, ns}); }
            }
            for (int[] e : adj[node]) {
                int v = e[0], w = e[1];
                if (phase == 0 && elev[v] > elev[node]) {
                    int ns = v * 2;
                    if (d + w < dist[ns]) { dist[ns] = d + w; pq.add(new long[]{d + w, ns}); }
                }
                if (phase == 1 && elev[v] < elev[node]) {
                    int ns = v * 2 + 1;
                    if (d + w < dist[ns]) { dist[ns] = d + w; pq.add(new long[]{d + w, ns}); }
                }
            }
        }
        System.out.println(dist[1]);
    }
}
