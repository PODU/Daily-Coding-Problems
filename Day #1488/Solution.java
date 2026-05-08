// Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
// Returns a valid ordering, or null if a cycle exists. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {

    // prereqs.get(course) = list of its prerequisites. Returns null if a cycle exists.
    static List<String> topoSort(Map<String, List<String>> prereqs) {
        Map<String, List<String>> adj = new HashMap<>();
        Map<String, Integer> indeg = new HashMap<>();
        for (Map.Entry<String, List<String>> e : prereqs.entrySet()) {
            indeg.putIfAbsent(e.getKey(), 0);
            for (String p : e.getValue()) indeg.putIfAbsent(p, 0);
        }
        for (Map.Entry<String, List<String>> e : prereqs.entrySet()) {
            for (String p : e.getValue()) {
                adj.computeIfAbsent(p, k -> new ArrayList<>()).add(e.getKey());
                indeg.merge(e.getKey(), 1, Integer::sum);
            }
        }
        PriorityQueue<String> q = new PriorityQueue<>(); // lexicographic for determinism
        for (Map.Entry<String, Integer> e : indeg.entrySet())
            if (e.getValue() == 0) q.add(e.getKey());

        List<String> order = new ArrayList<>();
        while (!q.isEmpty()) {
            String c = q.poll();
            order.add(c);
            for (String nxt : adj.getOrDefault(c, Collections.emptyList()))
                if (indeg.merge(nxt, -1, Integer::sum) == 0) q.add(nxt);
        }
        return order.size() == indeg.size() ? order : null;
    }

    public static void main(String[] args) {
        Map<String, List<String>> prereqs = new HashMap<>();
        prereqs.put("CSC300", Arrays.asList("CSC100", "CSC200"));
        prereqs.put("CSC200", Arrays.asList("CSC100"));
        prereqs.put("CSC100", new ArrayList<>());

        List<String> res = topoSort(prereqs);
        if (res == null) {
            System.out.println("null");
        } else {
            StringBuilder sb = new StringBuilder("[");
            for (int i = 0; i < res.size(); i++) {
                sb.append("'").append(res.get(i)).append("'");
                if (i + 1 < res.size()) sb.append(", ");
            }
            sb.append("]");
            System.out.println(sb.toString());
        }
    }
}
