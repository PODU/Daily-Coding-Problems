// Day 1445: Unit converter. Model units as a weighted graph (edge = conversion
// factor) and BFS for a path on query. addUnit O(1); convert O(V+E).
import java.util.*;

public class Solution {
    static class UnitConverter {
        // graph.get(a).get(b) = factor such that 1 a = factor b
        private final Map<String, Map<String, Double>> graph = new HashMap<>();

        void addUnit(String from, String to, double factor) {
            graph.computeIfAbsent(from, k -> new HashMap<>()).put(to, factor);
            graph.computeIfAbsent(to, k -> new HashMap<>()).put(from, 1.0 / factor);
        }

        double convert(double value, String from, String to) {
            if (from.equals(to)) return value;
            Map<String, Double> dist = new HashMap<>();
            Deque<String> q = new ArrayDeque<>();
            q.add(from); dist.put(from, 1.0);
            while (!q.isEmpty()) {
                String u = q.poll();
                for (Map.Entry<String, Double> e : graph.getOrDefault(u, Map.of()).entrySet()) {
                    if (!dist.containsKey(e.getKey())) {
                        dist.put(e.getKey(), dist.get(u) * e.getValue());
                        if (e.getKey().equals(to)) return value * dist.get(e.getKey());
                        q.add(e.getKey());
                    }
                }
            }
            return Double.NaN;
        }
    }

    public static void main(String[] args) {
        UnitConverter uc = new UnitConverter();
        uc.addUnit("foot", "inch", 12);
        uc.addUnit("yard", "foot", 3);
        uc.addUnit("chain", "yard", 22);
        System.out.println((long) uc.convert(1, "yard", "inch")); // 36
    }
}
