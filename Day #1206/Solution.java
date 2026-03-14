// Day 1206: Validate directional (N/S/E/W) rules for consistency.
// Split into vertical & horizontal strict-order graphs; a cycle = contradiction. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static class Graph {
        Map<String, List<String>> adj = new LinkedHashMap<>();
        Map<String, Integer> color = new HashMap<>();
        void add(String u, String v) {
            adj.computeIfAbsent(u, k -> new ArrayList<>()).add(v);
            adj.computeIfAbsent(v, k -> new ArrayList<>());
        }
        boolean dfs(String u) {
            color.put(u, 1);
            for (String v : adj.get(u)) {
                int c = color.getOrDefault(v, 0);
                if (c == 1) return true;
                if (c == 0 && dfs(v)) return true;
            }
            color.put(u, 2);
            return false;
        }
        boolean hasCycle() {
            for (String u : adj.keySet())
                if (color.getOrDefault(u, 0) == 0 && dfs(u)) return true;
            return false;
        }
    }

    static boolean validate(String[][] rules) {
        Graph gy = new Graph(), gx = new Graph();
        for (String[] r : rules) {
            String a = r[0], d = r[1], b = r[2];
            if (d.indexOf('N') >= 0) gy.add(a, b);
            if (d.indexOf('S') >= 0) gy.add(b, a);
            if (d.indexOf('E') >= 0) gx.add(a, b);
            if (d.indexOf('W') >= 0) gx.add(b, a);
        }
        return !gy.hasCycle() && !gx.hasCycle();
    }

    public static void main(String[] args) {
        String[][] rules = {{"A","N","B"}, {"B","NE","C"}, {"C","N","A"}};
        System.out.println(validate(rules) ? "validates" : "does not validate"); // does not validate
    }
}
