// Day 700: Cheapest itinerary with at most k connections (k+1 flights).
// Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
// track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).
import java.util.*;

public class Solution {
    static class Flight { String u, v; int w; Flight(String u, String v, int w) { this.u = u; this.v = v; this.w = w; } }

    static Object[] cheapest(List<Flight> flights, String src, String dst, int k) {
        Map<String, Integer> dist = new HashMap<>();
        Map<String, String> par = new HashMap<>();
        dist.put(src, 0);
        for (int it = 0; it <= k; it++) {            // up to k+1 edges
            Map<String, Integer> nd = new HashMap<>(dist);
            Map<String, String> np = new HashMap<>(par);
            for (Flight f : flights) {
                if (dist.containsKey(f.u)) {
                    int cand = dist.get(f.u) + f.w;
                    if (!nd.containsKey(f.v) || cand < nd.get(f.v)) { nd.put(f.v, cand); np.put(f.v, f.u); }
                }
            }
            dist = nd; par = np;
        }
        if (!dist.containsKey(dst)) return new Object[]{-1, new ArrayList<String>()};
        LinkedList<String> path = new LinkedList<>();
        String cur = dst;
        while (!cur.equals(src)) { path.addFirst(cur); cur = par.get(cur); }
        path.addFirst(src);
        return new Object[]{dist.get(dst), path};
    }

    public static void main(String[] args) {
        List<Flight> flights = Arrays.asList(
            new Flight("JFK", "ATL", 150), new Flight("ATL", "SFO", 400),
            new Flight("ORD", "LAX", 200), new Flight("LAX", "DFW", 80),
            new Flight("JFK", "HKG", 800), new Flight("ATL", "ORD", 90),
            new Flight("JFK", "LAX", 500));
        Object[] r = cheapest(flights, "JFK", "LAX", 3);
        @SuppressWarnings("unchecked") List<String> path = (List<String>) r[1];
        System.out.println(String.join(" -> ", path) + ", costing $" + r[0]);
        // JFK -> ATL -> ORD -> LAX, costing $440
    }
}
