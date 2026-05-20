// Simulate a Markov chain for num_steps and count visits per state.
// Time O(num_steps * outdegree), Space O(states + transitions).
import java.util.*;

public class Solution {
    static Map<String,Integer> runChain(String start, int numSteps,
            Map<String, List<Object[]>> trans, long seed) {
        Map<String,Integer> counts = new TreeMap<>();
        Random rng = new Random(seed);
        String cur = start;
        for (int s = 0; s < numSteps; s++) {
            counts.merge(cur, 1, Integer::sum);
            double r = rng.nextDouble(), acc = 0;
            for (Object[] e : trans.get(cur)) {
                acc += (double) e[1];
                if (r <= acc) { cur = (String) e[0]; break; }
            }
        }
        return counts;
    }

    public static void main(String[] args) {
        Map<String, List<Object[]>> trans = new HashMap<>();
        trans.put("a", List.of(new Object[]{"a",0.9}, new Object[]{"b",0.075}, new Object[]{"c",0.025}));
        trans.put("b", List.of(new Object[]{"a",0.15}, new Object[]{"b",0.8}, new Object[]{"c",0.05}));
        trans.put("c", List.of(new Object[]{"a",0.25}, new Object[]{"b",0.25}, new Object[]{"c",0.5}));
        Map<String,Integer> counts = runChain("a", 5000, trans, 42L);
        StringBuilder sb = new StringBuilder("{ ");
        boolean first = true;
        for (var e : counts.entrySet()) {
            if (!first) sb.append(", ");
            sb.append("'").append(e.getKey()).append("': ").append(e.getValue());
            first = false;
        }
        sb.append(" }");
        System.out.println(sb);
    }
}
