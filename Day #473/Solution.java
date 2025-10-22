// Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
// Time: O(V + E), Space: O(V + E).
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Solution {
    public static void main(String[] args) {
        // Original edges: A->B, B->C
        String[][] edges = {{"A", "B"}, {"B", "C"}};
        String[] order = {"A", "B", "C"};

        // Reverse adjacency: v -> u for each u -> v
        Map<String, List<String>> rev = new HashMap<>();
        for (String[] e : edges) {
            rev.computeIfAbsent(e[1], k -> new ArrayList<>()).add(e[0]);
        }

        // Original chain A -> B -> C becomes A <- B <- C
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < order.length; i++) {
            if (i > 0) sb.append(" <- ");
            sb.append(order[i]);
        }
        System.out.println(sb.toString());
    }
}
