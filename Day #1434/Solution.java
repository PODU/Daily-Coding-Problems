// Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
// Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
// Time: O((k+1) * E), Space: O(V).
import java.util.*;

public class Solution {
    static String cheapest(String[][] flights, String A, String B, int k) {
        Map<String,Integer> dist = new HashMap<>();
        Map<String,String> parent = new HashMap<>();
        for (String[] f : flights) {
            dist.putIfAbsent(f[0], Integer.MAX_VALUE);
            dist.putIfAbsent(f[1], Integer.MAX_VALUE);
        }
        dist.put(A, 0);
        for (int i = 0; i <= k; i++) {
            Map<String,Integer> cur = new HashMap<>(dist);
            Map<String,String> curParent = new HashMap<>(parent);
            for (String[] f : flights) {
                int ds = dist.get(f[0]);
                int price = Integer.parseInt(f[2]);
                if (ds != Integer.MAX_VALUE && ds + price < cur.get(f[1])) {
                    cur.put(f[1], ds + price);
                    curParent.put(f[1], f[0]);
                }
            }
            dist = cur; parent = curParent;
        }
        if (dist.get(B) == Integer.MAX_VALUE) return "No route";
        LinkedList<String> path = new LinkedList<>();
        String node = B;
        while (!node.equals(A)) { path.addFirst(node); node = parent.get(node); }
        path.addFirst(A);
        return String.join(" -> ", path) + ", costing $" + dist.get(B);
    }

    public static void main(String[] args) {
        String[][] flights = {
            {"JFK","ATL","150"}, {"ATL","SFO","400"}, {"ORD","LAX","200"},
            {"LAX","DFW","80"}, {"JFK","HKG","800"}, {"ATL","ORD","90"}, {"JFK","LAX","500"}
        };
        System.out.println(cheapest(flights, "JFK", "LAX", 3));
    }
}
