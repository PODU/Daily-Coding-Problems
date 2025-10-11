// Day 409: PageRank via power iteration with damping d=0.85.
// Approach: iterate score(j)=(1-d)/N + d*sum(score(i)/Ci); dangling rank
// is spread over all nodes. Time: O(iters*(N+E)), Space: O(N+E).
import java.util.*;

public class Solution {
    static Map<String, Double> pageRank(Map<String, List<String>> graph,
                                        double d, int iters, double eps) {
        int N = graph.size();
        Map<String, Double> rank = new TreeMap<>();
        for (String node : graph.keySet()) rank.put(node, 1.0 / N);
        for (int it = 0; it < iters; it++) {
            Map<String, Double> next = new TreeMap<>();
            double dangling = 0.0;
            for (Map.Entry<String, List<String>> e : graph.entrySet())
                if (e.getValue().isEmpty()) dangling += rank.get(e.getKey());
            for (String node : graph.keySet())
                next.put(node, (1.0 - d) / N + d * dangling / N);
            for (Map.Entry<String, List<String>> e : graph.entrySet()) {
                List<String> outs = e.getValue();
                if (outs.isEmpty()) continue;
                double share = rank.get(e.getKey()) / outs.size();
                for (String nbr : outs) next.put(nbr, next.get(nbr) + d * share);
            }
            double diff = 0.0;
            for (String node : graph.keySet())
                diff += Math.abs(next.get(node) - rank.get(node));
            rank = next;
            if (diff < eps) break;
        }
        return rank;
    }

    public static void main(String[] args) {
        Map<String, List<String>> graph = new TreeMap<>();
        graph.put("A", Arrays.asList("B", "C"));
        graph.put("B", Arrays.asList("C"));
        graph.put("C", Arrays.asList("A"));
        Map<String, Double> rank = pageRank(graph, 0.85, 100, 1e-12);
        for (Map.Entry<String, Double> e : rank.entrySet())
            System.out.printf("%s: %.4f%n", e.getKey(), e.getValue());
    }
}
