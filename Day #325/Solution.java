// Unit conversion via weighted graph; addConversion adds a<->b edges, convert does BFS multiplying ratios.
// Time: O(V+E) per query, Space: O(V+E).
import java.util.*;

public class Solution {
    static Map<String, List<double[]>> adjIdx = new HashMap<>();
    static Map<String, List<Object[]>> adj = new HashMap<>();

    static void addConversion(String a, String b, double ratio) {
        adj.computeIfAbsent(a, k -> new ArrayList<>()).add(new Object[]{b, ratio});
        adj.computeIfAbsent(b, k -> new ArrayList<>()).add(new Object[]{a, 1.0 / ratio});
    }

    static double convert(double value, String from, String to) {
        if (from.equals(to)) return value;
        Map<String, Double> dist = new HashMap<>();
        Deque<String> q = new ArrayDeque<>();
        q.add(from); dist.put(from, value);
        while (!q.isEmpty()) {
            String u = q.poll();
            for (Object[] e : adj.getOrDefault(u, Collections.emptyList())) {
                String v = (String) e[0];
                double w = (double) e[1];
                if (!dist.containsKey(v)) {
                    dist.put(v, dist.get(u) * w);
                    if (v.equals(to)) return dist.get(v);
                    q.add(v);
                }
            }
        }
        return dist.getOrDefault(to, Double.NaN);
    }

    public static void main(String[] args) {
        addConversion("foot", "inch", 12);
        addConversion("yard", "foot", 3);
        addConversion("chain", "yard", 22);
        double r = convert(1, "yard", "inch");
        System.out.printf("1 yard = %.1f inch%n", r);
    }
}
