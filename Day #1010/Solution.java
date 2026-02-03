// Unit Converter: graph where edge a->b stores factor (1 a = f b).
// convert() does BFS multiplying factors. Time O(V+E) per query, Space O(V+E).
import java.util.*;

public class Solution {
    static class UnitConverter {
        Map<String, List<Object[]>> g = new HashMap<>();

        void addUnit(String from, String to, double factor) {
            g.computeIfAbsent(from, k -> new ArrayList<>()).add(new Object[]{to, factor});
            g.computeIfAbsent(to, k -> new ArrayList<>()).add(new Object[]{from, 1.0 / factor});
        }

        double convert(double value, String from, String to) {
            if (from.equals(to)) return value;
            Map<String, Double> dist = new HashMap<>();
            dist.put(from, 1.0);
            Deque<String> q = new ArrayDeque<>();
            q.add(from);
            while (!q.isEmpty()) {
                String u = q.poll();
                for (Object[] e : g.getOrDefault(u, Collections.emptyList())) {
                    String v = (String) e[0];
                    double f = (double) e[1];
                    if (!dist.containsKey(v)) {
                        dist.put(v, dist.get(u) * f);
                        if (v.equals(to)) return value * dist.get(v);
                        q.add(v);
                    }
                }
            }
            return Double.NaN;
        }
    }

    public static void main(String[] args) {
        UnitConverter uc = new UnitConverter();
        uc.addUnit("inch", "foot", 1.0 / 12);
        uc.addUnit("foot", "yard", 1.0 / 3);
        uc.addUnit("yard", "chain", 1.0 / 22);

        System.out.println("1 chain = " + Math.round(uc.convert(1, "chain", "inch")) + " inches");
        System.out.println("36 inches = " + Math.round(uc.convert(36, "inch", "yard")) + " yards");
    }
}
