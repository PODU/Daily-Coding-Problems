// Day 346: Cheapest itinerary with up to k connections.
// Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
import java.util.*;

public class Solution {
    static class Entry { int cost; List<String> path; Entry(int c, List<String> p){cost=c;path=p;} }

    static Entry cheapest(String[][] flights, String src, String dst, int k) {
        Map<String, Entry> best = new HashMap<>();
        best.put(src, new Entry(0, new ArrayList<>(List.of(src))));
        for (int it = 0; it <= k; it++) {
            Map<String, Entry> nxt = new HashMap<>();
            for (Map.Entry<String, Entry> e : best.entrySet())
                nxt.put(e.getKey(), new Entry(e.getValue().cost, e.getValue().path));
            for (String[] f : flights) {
                String u = f[0], v = f[1]; int w = Integer.parseInt(f[2]);
                Entry pu = best.get(u);
                if (pu == null) continue;
                int cost = pu.cost + w;
                Entry nv = nxt.get(v);
                if (nv == null || cost < nv.cost) {
                    List<String> p = new ArrayList<>(pu.path); p.add(v);
                    nxt.put(v, new Entry(cost, p));
                }
            }
            best = nxt;
        }
        return best.get(dst);
    }

    public static void main(String[] args) {
        String[][] flights = {
            {"JFK","ATL","150"},{"ATL","SFO","400"},{"ORD","LAX","200"},
            {"LAX","DFW","80"},{"JFK","HKG","800"},{"ATL","ORD","90"},{"JFK","LAX","500"}
        };
        Entry res = cheapest(flights, "JFK", "LAX", 3);
        System.out.println(String.join(" -> ", res.path) + ", costing $" + res.cost + ".");
    }
}
