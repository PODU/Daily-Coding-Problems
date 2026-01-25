// Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
// Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).
import java.util.*;

public class Solution {
    static List<String> path;
    static int total;
    static Map<String, List<String>> adj;

    static boolean dfs(String node) {
        if (path.size() == total + 1) return true;
        List<String> dests = adj.getOrDefault(node, Collections.emptyList());
        for (int i = 0; i < dests.size(); i++) {
            String d = dests.get(i);
            if (d == null) continue;
            dests.set(i, null);
            path.add(d);
            if (dfs(d)) return true;
            path.remove(path.size() - 1);
            dests.set(i, d);
        }
        return false;
    }

    static List<String> itinerary(String[][] flights, String start) {
        adj = new HashMap<>();
        for (String[] f : flights)
            adj.computeIfAbsent(f[0], k -> new ArrayList<>()).add(f[1]);
        for (List<String> l : adj.values()) Collections.sort(l);
        total = flights.length;
        path = new ArrayList<>();
        path.add(start);
        return dfs(start) ? path : null;
    }

    static String show(List<String> v) {
        if (v == null) return "null";
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("'").append(v.get(i)).append("'");
        }
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        System.out.println(show(itinerary(new String[][]{{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL")));
        System.out.println(show(itinerary(new String[][]{{"SFO","COM"},{"COM","YYZ"}}, "COM")));
        System.out.println(show(itinerary(new String[][]{{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A")));
    }
}
