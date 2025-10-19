// Day 458: Validate directional (NE/SW/...) rules for consistency.
// Decompose into x/y strict orders; a cycle in either graph = contradiction.
// Time O(R + V) via DFS cycle detection.
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Solution {
    static class Graph {
        Map<String, List<String>> adj = new HashMap<>();
        Map<String, Integer> color = new HashMap<>();

        void addLess(String small, String big) {
            adj.computeIfAbsent(small, k -> new ArrayList<>()).add(big);
            color.putIfAbsent(small, 0);
            color.putIfAbsent(big, 0);
        }

        boolean dfs(String u) {
            color.put(u, 1);
            for (String v : adj.getOrDefault(u, List.of())) {
                int cv = color.getOrDefault(v, 0);
                if (cv == 1) return true;
                if (cv == 0 && dfs(v)) return true;
            }
            color.put(u, 2);
            return false;
        }

        boolean hasCycle() {
            for (String u : new ArrayList<>(color.keySet()))
                if (color.get(u) == 0 && dfs(u)) return true;
            return false;
        }
    }

    static boolean validate(String[] rules) {
        Graph gx = new Graph(), gy = new Graph();
        for (String r : rules) {
            String[] p = r.split("\\s+");
            String a = p[0], d = p[1], b = p[2];
            for (char c : d.toCharArray()) {
                if (c == 'N') gy.addLess(b, a);
                else if (c == 'S') gy.addLess(a, b);
                else if (c == 'E') gx.addLess(b, a);
                else if (c == 'W') gx.addLess(a, b);
            }
        }
        return !gx.hasCycle() && !gy.hasCycle();
    }

    public static void main(String[] args) {
        System.out.println(validate(new String[]{"A N B", "B NE C", "C N A"}) ? "Valid" : "Invalid"); // Invalid
        System.out.println(validate(new String[]{"A NW B", "A N B"}) ? "Valid" : "Invalid"); // Valid
    }
}
