// Shortest uphill-then-downhill cycle from home (node 0).
// Dijkstra on two DAG subgraphs (uphill, reversed downhill), O(E log V) time, O(V+E) space.
import java.util.*;

public class Solution {
    static long[] dijkstra(List<int[]>[] adj, int src, int n) {
        long[] dist = new long[n];
        Arrays.fill(dist, Long.MAX_VALUE);
        dist[src] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, src});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0];
            int u = (int) top[1];
            if (d > dist[u]) continue;
            for (int[] e : adj[u]) {
                int v = e[0], w = e[1];
                if (d + w < dist[v]) {
                    dist[v] = d + w;
                    pq.add(new long[]{dist[v], v});
                }
            }
        }
        return dist;
    }

    public static void main(String[] args) {
        Map<Integer, Integer> elevations = new HashMap<>();
        elevations.put(0, 5); elevations.put(1, 25); elevations.put(2, 15);
        elevations.put(3, 20); elevations.put(4, 10);
        int[][] paths = {{0,1,10},{0,2,8},{0,3,15},{1,3,12},
                         {2,4,10},{3,4,5},{3,0,17},{4,0,10}};
        int n = 0;
        for (int k : elevations.keySet()) n = Math.max(n, k + 1);

        @SuppressWarnings("unchecked")
        List<int[]>[] up = new List[n];
        @SuppressWarnings("unchecked")
        List<int[]>[] downRev = new List[n];
        for (int i = 0; i < n; i++) { up[i] = new ArrayList<>(); downRev[i] = new ArrayList<>(); }
        for (int[] p : paths) {
            int u = p[0], v = p[1], w = p[2];
            if (elevations.get(v) > elevations.get(u)) up[u].add(new int[]{v, w});
            else if (elevations.get(v) < elevations.get(u)) downRev[v].add(new int[]{u, w});
        }
        long[] upD = dijkstra(up, 0, n);
        long[] dnD = dijkstra(downRev, 0, n);
        long best = Long.MAX_VALUE;
        for (int p = 1; p < n; p++)
            if (upD[p] != Long.MAX_VALUE && dnD[p] != Long.MAX_VALUE)
                best = Math.min(best, upD[p] + dnD[p]);
        System.out.println(best);
    }
}
