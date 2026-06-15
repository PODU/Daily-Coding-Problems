// Day 1667: Course ordering via topological sort (Kahn's algorithm).
// Time O(V+E), Space O(V+E). Returns null if a cycle exists.
import java.util.*;

public class Solution {
    static List<String> courseOrder(Map<String, List<String>> prereqs) {
        Map<String, Integer> indeg = new HashMap<>();
        Map<String, List<String>> adj = new HashMap<>();
        for (var e : prereqs.entrySet()) {
            indeg.putIfAbsent(e.getKey(), 0);
            for (String d : e.getValue()) indeg.putIfAbsent(d, 0);
        }
        for (var e : prereqs.entrySet())
            for (String d : e.getValue()) {
                adj.computeIfAbsent(d, k -> new ArrayList<>()).add(e.getKey());
                indeg.merge(e.getKey(), 1, Integer::sum);
            }
        PriorityQueue<String> pq = new PriorityQueue<>();
        for (var e : indeg.entrySet()) if (e.getValue() == 0) pq.add(e.getKey());
        List<String> order = new ArrayList<>();
        while (!pq.isEmpty()) {
            String c = pq.poll();
            order.add(c);
            for (String nxt : adj.getOrDefault(c, List.of()))
                if (indeg.merge(nxt, -1, Integer::sum) == 0) pq.add(nxt);
        }
        return order.size() == indeg.size() ? order : null;
    }

    public static void main(String[] args) {
        Map<String, List<String>> prereqs = new HashMap<>();
        prereqs.put("CSC300", List.of("CSC100", "CSC200"));
        prereqs.put("CSC200", List.of("CSC100"));
        prereqs.put("CSC100", List.of());
        System.out.println(courseOrder(prereqs)); // [CSC100, CSC200, CSC300]
    }
}
