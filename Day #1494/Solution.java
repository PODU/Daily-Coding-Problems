// Day 1494: Reverse a directed graph by reversing every edge.
// Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        // Input graph: A -> B -> C
        String[][] edges = {{"A","B"},{"B","C"}};

        // Build reversed adjacency list.
        Map<String, List<String>> rev = new LinkedHashMap<>();
        for (String[] e : edges)
            rev.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]); // v -> u

        System.out.println("Original: A -> B -> C");
        System.out.println("Reversed: A <- B <- C");

        System.out.println("Reversed edges:");
        for (String[] e : edges)
            System.out.println("  " + e[1] + " -> " + e[0]);
    }
}
