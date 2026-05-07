// Day 1483: Shortest closed route from home (0) strictly ascending then
// descending. up[v]: shortest ascending 0->v; down[v]: shortest descending v->0
// (Dijkstra from 0 on reversed descending graph). Answer = min up[v]+down[v].
// Time O((V+E) log V), Space O(V+E).
import java.util.*;

public class Solution {
    static final long INF = Long.MAX_VALUE;

    static void dijkstra(int n, List<long[]>[] adj, int src, long[] dist, int[] pred) {
        Arrays.fill(dist, INF);
        Arrays.fill(pred, -1);
        dist[src] = 0;
        PriorityQueue<long[]> pq = new PriorityQueue<>((a, b) -> Long.compare(a[0], b[0]));
        pq.add(new long[]{0, src});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0]; int u = (int) top[1];
            if (d > dist[u]) continue;
            for (long[] e : adj[u]) {
                int v = (int) e[0]; long w = e[1];
                if (d + w < dist[v]) {
                    dist[v] = d + w;
                    pred[v] = u;
                    pq.add(new long[]{dist[v], v});
                }
            }
        }
    }

    @SuppressWarnings("unchecked")
    public static void main(String[] args) {
        int n = 5;
        int[] elev = {5, 25, 15, 20, 10};
        long[][] edges = {{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
                {2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}};

        List<long[]>[] upAdj = new List[n], revDesc = new List[n];
        for (int i = 0; i < n; ++i) { upAdj[i] = new ArrayList<>(); revDesc[i] = new ArrayList<>(); }
        for (long[] e : edges) {
            int a = (int) e[0], b = (int) e[1]; long w = e[2];
            if (elev[b] > elev[a]) upAdj[a].add(new long[]{b, w});
            else if (elev[b] < elev[a]) revDesc[b].add(new long[]{a, w});
        }

        long[] up = new long[n], down = new long[n];
        int[] upPred = new int[n], downPred = new int[n];
        dijkstra(n, upAdj, 0, up, upPred);
        dijkstra(n, revDesc, 0, down, downPred);

        long best = INF; int peak = -1;
        for (int v = 1; v < n; ++v)
            if (up[v] != INF && down[v] != INF && up[v] > 0 && down[v] > 0 && up[v] + down[v] < best) {
                best = up[v] + down[v]; peak = v;
            }

        LinkedList<Integer> route = new LinkedList<>();
        for (int c = peak; c != -1; c = upPred[c]) route.addFirst(c);
        for (int c = downPred[peak]; c != -1; c = downPred[c]) route.addLast(c);

        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < route.size(); ++i) sb.append(i > 0 ? " -> " : "").append(route.get(i));
        System.out.println("The shortest valid path would be " + sb + ", with a distance of " + best + ".");
        // The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
    }
}
