// PageRank: iterative power method with damping d=0.85, dangling-node mass redistributed evenly.
// Time O(iters*(N+E)), Space O(N+E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] nodes = {"A", "B", "C", "D"};
        Map<String, List<String>> edges = new HashMap<>();
        edges.put("A", Arrays.asList("B", "C"));
        edges.put("B", Arrays.asList("C"));
        edges.put("C", Arrays.asList("A"));
        edges.put("D", Arrays.asList("C"));
        int N = nodes.length;
        double d = 0.85;
        Map<String, Double> score = new HashMap<>();
        for (String nd : nodes) score.put(nd, 1.0 / N);

        for (int it = 0; it < 100; it++) {
            Map<String, Double> next = new HashMap<>();
            double dangling = 0.0;
            for (String nd : nodes)
                if (edges.getOrDefault(nd, Collections.emptyList()).isEmpty())
                    dangling += score.get(nd);
            for (String nd : nodes)
                next.put(nd, (1.0 - d) / N + d * dangling / N);
            for (String nd : nodes) {
                List<String> outs = edges.getOrDefault(nd, Collections.emptyList());
                if (outs.isEmpty()) continue;
                double share = d * score.get(nd) / outs.size();
                for (String t : outs) next.put(t, next.get(t) + share);
            }
            double diff = 0.0;
            for (String nd : nodes) diff += Math.abs(next.get(nd) - score.get(nd));
            score = next;
            if (diff < 1e-9) break;
        }

        for (String nd : nodes)
            System.out.printf("%s %.6f%n", nd, score.get(nd));
    }
}
