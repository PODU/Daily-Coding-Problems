// Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
// edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
import java.util.*;

public class Solution {
    static class Graph {
        Map<String, Set<String>> adj = new HashMap<>();
        void edge(String a, String b) {
            adj.computeIfAbsent(a, k -> new HashSet<>()).add(b);
            adj.computeIfAbsent(b, k -> new HashSet<>());
        }
        boolean util(String u, Map<String, Integer> state) {
            state.put(u, 1);
            for (String v : adj.get(u)) {
                int s = state.getOrDefault(v, 0);
                if (s == 1) return true;
                if (s == 0 && util(v, state)) return true;
            }
            state.put(u, 2);
            return false;
        }
        boolean hasCycle() {
            Map<String, Integer> state = new HashMap<>();
            for (String u : adj.keySet())
                if (state.getOrDefault(u, 0) == 0 && util(u, state)) return true;
            return false;
        }
    }

    static boolean validate(String[][] rules) {
        Graph vert = new Graph(), horiz = new Graph();
        for (String[] r : rules) {
            String a = r[0], dir = r[1], b = r[2];
            for (char c : dir.toCharArray()) {
                if (c == 'N') vert.edge(a, b);
                else if (c == 'S') vert.edge(b, a);
                else if (c == 'E') horiz.edge(a, b);
                else if (c == 'W') horiz.edge(b, a);
            }
        }
        return !vert.hasCycle() && !horiz.hasCycle();
    }

    public static void main(String[] args) {
        String[][] rules = {{"A","N","B"},{"B","NE","C"},{"C","N","A"}};
        System.out.println(validate(rules) ? "validates" : "does not validate");
        // does not validate
    }
}
