// Uphill-then-downhill closed route: Dijkstra on up-edges from 0, Dijkstra on reversed down-edges
// to 0; answer = min over peaks of distUp[m]+distDown[m]. Time O((V+E)logV), Space O(V+E).
import java.util.*;

public class Solution {
    static long[] dijkstra(int n, List<int[]>[] adj, int src) {
        long[] d = new long[n];
        Arrays.fill(d, Long.MAX_VALUE);
        d[src] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, src});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long du = top[0]; int u = (int) top[1];
            if (du > d[u]) continue;
            for (int[] e : adj[u]) {
                int v = e[0], w = e[1];
                if (d[u] + w < d[v]) { d[v] = d[u] + w; pq.add(new long[]{d[v], v}); }
            }
        }
        return d;
    }

    public static void main(String[] args) {
        int n = 5;
        int[] elev = {5, 25, 15, 20, 10};
        int[][] paths = {{0,1,10},{0,2,8},{0,3,15},{1,3,12},{2,4,10},{3,4,5},{3,0,17},{4,0,10}};
        List<int[]>[] up = new List[n], downRev = new List[n];
        for (int i = 0; i < n; i++) { up[i] = new ArrayList<>(); downRev[i] = new ArrayList<>(); }
        for (int[] p : paths) {
            int u = p[0], v = p[1], w = p[2];
            if (elev[v] > elev[u]) up[u].add(new int[]{v, w});
            if (elev[v] < elev[u]) downRev[v].add(new int[]{u, w});
        }
        long[] distUp = dijkstra(n, up, 0);
        long[] distDown = dijkstra(n, downRev, 0);
        long best = Long.MAX_VALUE;
        for (int m = 1; m < n; m++)
            if (distUp[m] != Long.MAX_VALUE && distDown[m] != Long.MAX_VALUE)
                best = Math.min(best, distUp[m] + distDown[m]);
        System.out.println(best);
    }
}
