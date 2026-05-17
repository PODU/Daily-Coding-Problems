// Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
// Returns total cost to connect every house to the plant.
// Time O(E log V), Space O(V + E).
import java.util.*;

public class Solution {
    static int mstCost(Map<String, Map<String,Integer>> g) {
        Map<String, List<int[]>> idx = new HashMap<>(); // not used; build adj of names via list
        Map<String, List<Object[]>> adj = new HashMap<>();
        for (var e : g.entrySet()) {
            adj.putIfAbsent(e.getKey(), new ArrayList<>());
            for (var nb : e.getValue().entrySet()) {
                adj.computeIfAbsent(e.getKey(), k -> new ArrayList<>()).add(new Object[]{nb.getKey(), nb.getValue()});
                adj.computeIfAbsent(nb.getKey(), k -> new ArrayList<>()).add(new Object[]{e.getKey(), nb.getValue()});
            }
        }
        Set<String> visited = new HashSet<>();
        PriorityQueue<Object[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> (int)a[0]));
        String start = g.keySet().iterator().next();
        pq.add(new Object[]{0, start});
        int total = 0;
        while (!pq.isEmpty()) {
            Object[] cur = pq.poll();
            int w = (int) cur[0];
            String u = (String) cur[1];
            if (visited.contains(u)) continue;
            visited.add(u);
            total += w;
            for (Object[] nb : adj.getOrDefault(u, List.of())) {
                String v = (String) nb[0];
                int cw = (int) nb[1];
                if (!visited.contains(v)) pq.add(new Object[]{cw, v});
            }
        }
        return total;
    }

    public static void main(String[] args) {
        Map<String, Map<String,Integer>> pipes = new LinkedHashMap<>();
        pipes.put("plant", Map.of("A",1,"B",5,"C",20));
        pipes.put("A", Map.of("C",15));
        pipes.put("B", Map.of("C",10));
        pipes.put("C", Map.of());
        System.out.println(mstCost(pipes));
    }
}
