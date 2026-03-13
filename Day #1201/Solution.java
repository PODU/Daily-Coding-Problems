// Day 1201: Reverse a directed graph.
// Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static Map<String, List<String>> reverseGraph(Map<String, List<String>> g) {
        Map<String, List<String>> r = new LinkedHashMap<>();
        for (String u : g.keySet()) r.putIfAbsent(u, new ArrayList<>());
        for (Map.Entry<String, List<String>> e : g.entrySet())
            for (String v : e.getValue())
                r.computeIfAbsent(v, k -> new ArrayList<>()).add(e.getKey());
        return r;
    }

    public static void main(String[] args) {
        String[] nodes = {"A", "B", "C"};
        Map<String, List<String>> g = new LinkedHashMap<>();
        for (int i = 0; i + 1 < nodes.length; i++)
            g.computeIfAbsent(nodes[i], k -> new ArrayList<>()).add(nodes[i + 1]);
        reverseGraph(g);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < nodes.length; i++) {
            if (i > 0) sb.append(" <- ");
            sb.append(nodes[i]);
        }
        System.out.println(sb);
    }
}
