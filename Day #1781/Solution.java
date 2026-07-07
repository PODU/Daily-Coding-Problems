// Unit converter as a graph: addRelation stores 1 a = factor b (edge a->b, b->a=1/factor).
// convert does BFS multiplying factors along the path. Time O(V+E) per query, Space O(V+E).
import java.util.*;

public class Solution {
    static class UnitConverter {
        private final Map<String, List<double[]>> idx = new HashMap<>();
        private final List<String> names = new ArrayList<>();
        private final Map<String, Map<String, Double>> adj = new HashMap<>();

        void addRelation(String a, String b, double factor) {
            adj.computeIfAbsent(a, k -> new HashMap<>()).put(b, factor);
            adj.computeIfAbsent(b, k -> new HashMap<>()).put(a, 1.0 / factor);
        }

        Double convert(double qty, String from, String to) {
            if (from.equals(to)) return qty;
            if (!adj.containsKey(from) || !adj.containsKey(to)) return null;
            Map<String, Double> dist = new HashMap<>();
            Deque<String> q = new ArrayDeque<>();
            q.add(from); dist.put(from, qty);
            while (!q.isEmpty()) {
                String u = q.poll();
                for (Map.Entry<String, Double> e : adj.get(u).entrySet()) {
                    if (!dist.containsKey(e.getKey())) {
                        double v = dist.get(u) * e.getValue();
                        dist.put(e.getKey(), v);
                        if (e.getKey().equals(to)) return v;
                        q.add(e.getKey());
                    }
                }
            }
            return null;
        }
    }

    public static void main(String[] args) {
        UnitConverter uc = new UnitConverter();
        uc.addRelation("inches", "foot", 1.0 / 12.0);
        uc.addRelation("feet", "yard", 1.0 / 3.0);
        uc.addRelation("yards", "chain", 1.0 / 22.0);
        uc.addRelation("foot", "feet", 1.0);
        uc.addRelation("yard", "yards", 1.0);

        System.out.printf("1 yard = %.0f inches%n", uc.convert(1, "yard", "inches"));
        System.out.printf("1 chain = %.0f inches%n", uc.convert(1, "chain", "inches"));
        System.out.printf("1 chain = %.0f feet%n", uc.convert(1, "chain", "feet"));
    }
}
