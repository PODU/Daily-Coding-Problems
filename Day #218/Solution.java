// Day 218: Reverse a directed graph (transpose).
// Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static Map<String, List<String>> reverseGraph(Map<String, List<String>> g) {
        Map<String, List<String>> r = new LinkedHashMap<>();
        for (String u : g.keySet()) r.putIfAbsent(u, new ArrayList<>());
        for (Map.Entry<String, List<String>> e : g.entrySet()) {
            for (String v : e.getValue()) {
                r.computeIfAbsent(v, k -> new ArrayList<>()).add(e.getKey());
            }
        }
        return r;
    }

    public static void main(String[] args) {
        Map<String, List<String>> g = new LinkedHashMap<>();
        g.put("A", List.of("B"));
        g.put("B", List.of("C"));
        g.put("C", List.of());
        Map<String, List<String>> r = reverseGraph(g);
        // Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
        System.out.println("A <- B <- C");
    }
}
