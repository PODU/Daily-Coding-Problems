// Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
// Returns a valid course order or null if a cycle exists. O(V+E).
import java.util.*;

public class Solution {
    static List<String> courseOrder(Map<String, List<String>> prereqs) {
        Map<String, Integer> indeg = new HashMap<>();
        Map<String, List<String>> adj = new HashMap<>();
        for (String c : prereqs.keySet()) indeg.put(c, 0);
        for (var e : prereqs.entrySet())
            for (String p : e.getValue()) {
                adj.computeIfAbsent(p, k -> new ArrayList<>()).add(e.getKey());
                indeg.merge(e.getKey(), 1, Integer::sum);
            }
        PriorityQueue<String> q = new PriorityQueue<>();
        for (var e : indeg.entrySet()) if (e.getValue() == 0) q.add(e.getKey());
        List<String> order = new ArrayList<>();
        while (!q.isEmpty()) {
            String c = q.poll();
            order.add(c);
            for (String n : adj.getOrDefault(c, List.of()))
                if (indeg.merge(n, -1, Integer::sum) == 0) q.add(n);
        }
        return order.size() == prereqs.size() ? order : null;
    }

    public static void main(String[] args) {
        Map<String, List<String>> g = new HashMap<>();
        g.put("CSC300", List.of("CSC100", "CSC200"));
        g.put("CSC200", List.of("CSC100"));
        g.put("CSC100", List.of());
        System.out.println(courseOrder(g)); // [CSC100, CSC200, CSC300]
    }
}
