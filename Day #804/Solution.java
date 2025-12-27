// Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
// Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
// Time O((k+1) * E), Space O(V + E).
import java.util.*;

public class Solution {
    static class Flight {
        String src, dst; int price;
        Flight(String s, String d, int p) { src = s; dst = d; price = p; }
    }

    static Object[] cheapest(List<Flight> flights, String A, String B, int k) {
        final int INF = Integer.MAX_VALUE;
        Map<String, Integer> dist = new HashMap<>();
        Map<String, String> parent = new HashMap<>();
        for (Flight f : flights) { dist.put(f.src, INF); dist.put(f.dst, INF); }
        dist.put(A, 0);
        for (int it = 0; it <= k; it++) {
            Map<String, Integer> snap = new HashMap<>(dist);
            for (Flight f : flights) {
                if (snap.get(f.src) == INF) continue;
                if (snap.get(f.src) + f.price < dist.get(f.dst)) {
                    dist.put(f.dst, snap.get(f.src) + f.price);
                    parent.put(f.dst, f.src);
                }
            }
        }
        if (dist.get(B) == INF) return new Object[]{-1, Collections.emptyList()};
        LinkedList<String> path = new LinkedList<>();
        for (String c = B; ; c = parent.get(c)) {
            path.addFirst(c);
            if (c.equals(A)) break;
        }
        return new Object[]{dist.get(B), path};
    }

    public static void main(String[] args) {
        List<Flight> flights = Arrays.asList(
            new Flight("JFK", "ATL", 150), new Flight("ATL", "SFO", 400),
            new Flight("ORD", "LAX", 200), new Flight("LAX", "DFW", 80),
            new Flight("JFK", "HKG", 800), new Flight("ATL", "ORD", 90),
            new Flight("JFK", "LAX", 500));
        Object[] r = cheapest(flights, "JFK", "LAX", 3);
        @SuppressWarnings("unchecked")
        List<String> path = (List<String>) r[1];
        System.out.println(String.join(" -> ", path) + ", costing $" + r[0]);
        // JFK -> ATL -> ORD -> LAX, costing $440
    }
}
