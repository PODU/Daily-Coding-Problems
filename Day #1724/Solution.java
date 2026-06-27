// Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
// Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
// Time: O(k * E), Space: O(V).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[][] flights = {
            {"JFK", "ATL", "150"}, {"ATL", "SFO", "400"}, {"ORD", "LAX", "200"},
            {"LAX", "DFW", "80"},  {"JFK", "HKG", "800"}, {"ATL", "ORD", "90"},
            {"JFK", "LAX", "500"},
        };
        String src = "JFK", dst = "LAX";
        int k = 3; // max flights (edges)

        final int INF = Integer.MAX_VALUE;
        Map<String, Integer> dist = new HashMap<>();
        Map<String, String> parent = new HashMap<>();
        Set<String> cities = new TreeSet<>();
        for (String[] f : flights) { cities.add(f[0]); cities.add(f[1]); }
        for (String c : cities) dist.put(c, INF);
        dist.put(src, 0);

        for (int i = 0; i < k; i++) {
            Map<String, Integer> snap = new HashMap<>(dist);
            for (String[] f : flights) {
                int w = Integer.parseInt(f[2]);
                int du = snap.get(f[0]);
                if (du != INF && du + w < dist.get(f[1])) {
                    dist.put(f[1], du + w);
                    parent.put(f[1], f[0]);
                }
            }
        }

        if (dist.get(dst) == INF) { System.out.println("No route"); return; }
        List<String> path = new ArrayList<>();
        for (String at = dst; ; at = parent.get(at)) {
            path.add(at);
            if (at.equals(src)) break;
        }
        Collections.reverse(path);
        System.out.println(String.join(" -> ", path)
            + ", costing $" + dist.get(dst));
    }
}
