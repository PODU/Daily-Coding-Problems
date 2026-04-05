// Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
// Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
import java.util.*;

public class Solution {
    static int minCost(Map<String, Map<String,Integer>> g) {
        if (g.isEmpty()) return 0;
        Set<String> visited = new HashSet<>();
        PriorityQueue<int[]> pq = new PriorityQueue<>((a,b)->a[0]-b[0]);
        List<String> nodes = new ArrayList<>(g.keySet());
        Map<String,Integer> idx = new HashMap<>();
        for (int i = 0; i < nodes.size(); i++) idx.put(nodes.get(i), i);
        pq.add(new int[]{0, idx.get(nodes.get(0))});
        int total = 0;
        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            String u = nodes.get(cur[1]);
            if (visited.contains(u)) continue;
            visited.add(u);
            total += cur[0];
            for (Map.Entry<String,Integer> e : g.get(u).entrySet())
                if (!visited.contains(e.getKey()))
                    pq.add(new int[]{e.getValue(), idx.get(e.getKey())});
        }
        return total;
    }

    static void addEdge(Map<String, Map<String,Integer>> g, String a, String b, int c) {
        g.computeIfAbsent(a, k->new HashMap<>()).put(b, c);
        g.computeIfAbsent(b, k->new HashMap<>()).put(a, c);
    }

    public static void main(String[] args) {
        Map<String, Map<String,Integer>> g = new HashMap<>();
        addEdge(g, "plant", "A", 1);
        addEdge(g, "plant", "B", 5);
        addEdge(g, "plant", "C", 20);
        addEdge(g, "A", "C", 15);
        addEdge(g, "B", "C", 10);
        System.out.println(minCost(g)); // 16
    }
}
