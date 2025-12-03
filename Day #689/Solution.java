// Unit conversion as a weighted graph; convert via BFS multiplying edge ratios.
// add_conversion(a, b, r): 1 a = r b. Time O(V + E) per query, Space O(V + E).
import java.util.*;

public class Solution {
    static class UnitConverter {
        Map<String, Map<String, Double>> graph = new HashMap<>();

        void addConversion(String frm, String to, double factor) {
            // 1 frm = factor to
            graph.computeIfAbsent(frm, k -> new HashMap<>()).put(to, factor);
            graph.computeIfAbsent(to, k -> new HashMap<>()).put(frm, 1.0 / factor);
        }

        Double convert(double qty, String frm, String to) {
            if (frm.equals(to)) return qty;
            if (!graph.containsKey(frm) || !graph.containsKey(to)) return null;
            Set<String> visited = new HashSet<>();
            visited.add(frm);
            Deque<Object[]> queue = new ArrayDeque<>();
            queue.add(new Object[]{frm, 1.0});
            while (!queue.isEmpty()) {
                Object[] cur = queue.poll();
                String unit = (String) cur[0];
                double ratio = (Double) cur[1];
                if (unit.equals(to)) return qty * ratio;
                for (Map.Entry<String, Double> e : graph.get(unit).entrySet()) {
                    if (!visited.contains(e.getKey())) {
                        visited.add(e.getKey());
                        queue.add(new Object[]{e.getKey(), ratio * e.getValue()});
                    }
                }
            }
            return null;
        }
    }

    public static void main(String[] args) {
        UnitConverter uc = new UnitConverter();
        uc.addConversion("foot", "inch", 12);   // 12 inches = 1 foot
        uc.addConversion("yard", "foot", 3);    // 3 feet = 1 yard
        uc.addConversion("chain", "yard", 22);  // 22 yards = 1 chain

        long r1 = Math.round(uc.convert(1, "chain", "inch"));
        long r2 = Math.round(uc.convert(1, "yard", "inch"));
        System.out.println("1 chain = " + r1 + " inches");
        System.out.println("1 yard = " + r2 + " inches");
    }
}
