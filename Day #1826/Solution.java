// Dijkstra from node 0; answer is the max shortest-path distance (broadcast time).
// O((V+E) log V).
import java.util.*;

public class Solution {
    public static void main(String[] args){
        int[][] edges = {{0,1,5},{0,2,3},{0,5,4},{1,3,8},{2,3,1},{3,5,10},{3,4,5}};
        int maxNode = 0;
        for(int[] e : edges) maxNode = Math.max(maxNode, Math.max(e[0], e[1]));
        int V = maxNode + 1;
        List<int[]>[] adj = new List[V];
        for(int i = 0; i < V; i++) adj[i] = new ArrayList<>();
        for(int[] e : edges){ adj[e[0]].add(new int[]{e[1], e[2]}); adj[e[1]].add(new int[]{e[0], e[2]}); }

        long[] dist = new long[V];
        Arrays.fill(dist, Long.MAX_VALUE);
        PriorityQueue<long[]> pq = new PriorityQueue<>((a,b) -> Long.compare(a[0], b[0]));
        dist[0] = 0; pq.add(new long[]{0, 0});
        while(!pq.isEmpty()){
            long[] top = pq.poll();
            long d = top[0]; int u = (int) top[1];
            if(d > dist[u]) continue;
            for(int[] e : adj[u]){
                int v = e[0], w = e[1];
                if(d + w < dist[v]){ dist[v] = d + w; pq.add(new long[]{dist[v], v}); }
            }
        }
        long ans = 0;
        for(long d : dist) ans = Math.max(ans, d);
        System.out.println(ans); // 9
    }
}
