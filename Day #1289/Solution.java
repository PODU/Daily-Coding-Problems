// Day 1289: PageRank via power iteration with damping factor d.
// Iterate score vector until convergence; dangling nodes spread mass uniformly.
// Time O(iters * (V + E)), Space O(V + E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] nodes = {"A", "B", "C", "D"};
        Map<String, String[]> links = new HashMap<>();
        links.put("A", new String[]{"B", "C"});
        links.put("B", new String[]{"C"});
        links.put("C", new String[]{"A"});
        links.put("D", new String[]{"C"});
        double d = 0.85;
        int iters = 100, n = nodes.length;

        Map<String, Integer> outCount = new HashMap<>();
        for (String node : nodes) outCount.put(node, links.containsKey(node) ? links.get(node).length : 0);
        Map<String, Double> score = new HashMap<>();
        for (String node : nodes) score.put(node, 1.0 / n);

        for (int it = 0; it < iters; ++it) {
            double danglingSum = 0;
            for (String node : nodes) if (outCount.get(node) == 0) danglingSum += score.get(node);
            Map<String, Double> nw = new HashMap<>();
            for (String node : nodes) nw.put(node, (1 - d) / n + d * danglingSum / n);
            for (String src : nodes) {
                if (outCount.get(src) == 0) continue;
                double share = d * score.get(src) / outCount.get(src);
                for (String dst : links.get(src)) nw.put(dst, nw.get(dst) + share);
            }
            score = nw;
        }
        for (String node : nodes)
            System.out.printf("%s: %.4f%n", node, score.get(node));
    }
}
