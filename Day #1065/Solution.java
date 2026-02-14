// Day 1065: Reverse a directed graph (reverse every edge).
// Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static Map<String, List<String>> reverseGraph(Map<String, List<String>> g) {
        Map<String, List<String>> r = new TreeMap<>();
        for (String u : g.keySet()) r.putIfAbsent(u, new ArrayList<>()); // keep isolated nodes
        for (Map.Entry<String, List<String>> e : g.entrySet())
            for (String v : e.getValue())
                r.computeIfAbsent(v, k -> new ArrayList<>()).add(e.getKey());
        return r;
    }

    public static void main(String[] args) {
        // A -> B -> C
        Map<String, List<String>> g = new TreeMap<>();
        g.put("A", new ArrayList<>(List.of("B")));
        g.put("B", new ArrayList<>(List.of("C")));
        g.put("C", new ArrayList<>());

        Map<String, List<String>> r = reverseGraph(g);
        // Reversed: B -> A, C -> B  ("A <- B <- C")
        System.out.println("A <- B <- C");
        for (Map.Entry<String, List<String>> e : r.entrySet())
            for (String v : e.getValue())
                System.out.println(e.getKey() + " -> " + v);
    }
}
