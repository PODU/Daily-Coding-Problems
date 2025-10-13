// Day 427: Shortest uphill-then-downhill route from/to home (location 0).
// State Dijkstra: each node split into up/down phases; switch at the peak.
// Up edges need strictly higher elevation, down edges strictly lower. Time O((V+E)logV).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<Integer, Integer> elev = new HashMap<>();
        elev.put(0, 5); elev.put(1, 25); elev.put(2, 15); elev.put(3, 20); elev.put(4, 10);
        int[][] edges = {{0, 1, 10}, {0, 2, 8}, {0, 3, 15}, {1, 3, 12},
                         {2, 4, 10}, {3, 4, 5}, {3, 0, 17}, {4, 0, 10}};
        int n = elev.size(), home = 0;
        List<int[]>[] adj = new List[n];
        for (int i = 0; i < n; i++) adj[i] = new ArrayList<>();
        for (int[] e : edges) adj[e[0]].add(new int[]{e[1], e[2]});

        int S = n * 2; // state = node*2 + phase (0 up, 1 down)
        long[] dist = new long[S];
        Arrays.fill(dist, Long.MAX_VALUE);
        int[] prev = new int[S];
        Arrays.fill(prev, -1);
        PriorityQueue<long[]> pq = new PriorityQueue<>((x, y) -> Long.compare(x[0], y[0]));
        dist[home * 2] = 0;
        pq.add(new long[]{0, home * 2});
        while (!pq.isEmpty()) {
            long[] top = pq.poll();
            long d = top[0];
            int s = (int) top[1];
            if (d > dist[s]) continue;
            int u = s / 2, ph = s % 2;
            if (ph == 0 && u != home) {
                int ns = u * 2 + 1;
                if (d < dist[ns]) { dist[ns] = d; prev[ns] = s; pq.add(new long[]{d, ns}); }
            }
            for (int[] pr : adj[u]) {
                int v = pr[0], w = pr[1], ns;
                if (ph == 0 && elev.get(v) > elev.get(u)) ns = v * 2;
                else if (ph == 1 && elev.get(v) < elev.get(u)) ns = v * 2 + 1;
                else continue;
                if (d + w < dist[ns]) { dist[ns] = d + w; prev[ns] = s; pq.add(new long[]{d + w, ns}); }
            }
        }
        int goal = home * 2 + 1;
        List<Integer> nodes = new ArrayList<>();
        for (int cur = goal; cur != -1; cur = prev[cur]) nodes.add(cur / 2);
        Collections.reverse(nodes);
        List<Integer> path = new ArrayList<>();
        for (int x : nodes)
            if (path.isEmpty() || path.get(path.size() - 1) != x) path.add(x);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < path.size(); i++) {
            sb.append(path.get(i));
            if (i + 1 < path.size()) sb.append(" -> ");
        }
        sb.append(", distance ").append(dist[goal]);
        System.out.println(sb.toString());
    }
}
