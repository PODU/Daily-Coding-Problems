// Day 640: Simulate a Markov chain and tally state visits.
// Approach: build outgoing-edge table, draw next state by cumulative prob.
// Time: O(num_steps * avg_out_degree), Space: O(states + edges).
// Note: output is stochastic; counts approximate the README sample (sum 5000).
import java.util.*;

public class Solution {
    static Map<String, Integer> runMarkov(String start, Object[][] trans, int steps, long seed) {
        Map<String, List<Object[]>> adj = new HashMap<>();
        for (Object[] t : trans)
            adj.computeIfAbsent((String) t[0], k -> new ArrayList<>())
               .add(new Object[]{t[1], t[2]});
        Map<String, Integer> counts = new LinkedHashMap<>();
        Random rng = new Random(seed);
        String cur = start;
        for (int i = 0; i < steps; i++) {
            counts.merge(cur, 1, Integer::sum);
            double r = rng.nextDouble(), acc = 0;
            for (Object[] e : adj.get(cur)) {
                acc += (double) e[1];
                if (r <= acc) { cur = (String) e[0]; break; }
            }
        }
        return counts;
    }

    public static void main(String[] args) {
        Object[][] trans = {
            {"a","a",0.9},{"a","b",0.075},{"a","c",0.025},
            {"b","a",0.15},{"b","b",0.8},{"b","c",0.05},
            {"c","a",0.25},{"c","b",0.25},{"c","c",0.5}
        };
        Map<String, Integer> counts = runMarkov("a", trans, 5000, 42);
        TreeMap<String, Integer> sorted = new TreeMap<>(counts);
        StringBuilder sb = new StringBuilder("{ ");
        boolean first = true;
        for (Map.Entry<String, Integer> e : sorted.entrySet()) {
            if (!first) sb.append(", ");
            sb.append("'").append(e.getKey()).append("': ").append(e.getValue());
            first = false;
        }
        sb.append(" }");
        System.out.println(sb); // e.g. { 'a': 3012, 'b': 1656, 'c': 332 }
    }
}
