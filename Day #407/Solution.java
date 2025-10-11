// Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
// Approach: Prim with a min-heap over an undirected weighted graph.
// Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
import java.util.*;

public class Solution {
    static int minimumCost(Map<String, List<int[]>> adjIdx, List<String> nodes) {
        if (nodes.isEmpty()) return 0;
        Set<Integer> visited = new HashSet<>();
        // [cost, nodeIndex]
        PriorityQueue<int[]> pq = new PriorityQueue<>((a, b) -> a[0] - b[0]);
        pq.add(new int[]{0, 0});
        int total = 0;
        while (!pq.isEmpty()) {
            int[] cur = pq.poll();
            int cost = cur[0], node = cur[1];
            if (visited.contains(node)) continue;
            visited.add(node);
            total += cost;
            for (int[] e : adjIdx.get(nodes.get(node))) {
                if (!visited.contains(e[0])) pq.add(new int[]{e[1], e[0]});
            }
        }
        return total;
    }

    public static void main(String[] args) {
        List<String> nodes = Arrays.asList("plant", "A", "B", "C");
        Map<String, Integer> idx = new HashMap<>();
        for (int i = 0; i < nodes.size(); i++) idx.put(nodes.get(i), i);
        int[][] edges = {{0,1,1},{0,2,5},{0,3,20},{1,3,15},{2,3,10}};
        Map<String, List<int[]>> adj = new HashMap<>();
        for (String n : nodes) adj.put(n, new ArrayList<>());
        for (int[] e : edges) {
            adj.get(nodes.get(e[0])).add(new int[]{e[1], e[2]});
            adj.get(nodes.get(e[1])).add(new int[]{e[0], e[2]});
        }
        System.out.println(minimumCost(adj, nodes)); // 16
    }
}
