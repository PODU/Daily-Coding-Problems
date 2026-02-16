// Markov chain simulation: seeded java.util.Random; O(steps*states) time O(states^2) space
// Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
// Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<String, List<Object[]>> trans = new LinkedHashMap<>();
        Object[][] transitions = {
            {"a", "a", 0.9},   {"a", "b", 0.075}, {"a", "c", 0.025},
            {"b", "a", 0.15},  {"b", "b", 0.8},   {"b", "c", 0.05},
            {"c", "a", 0.25},  {"c", "b", 0.25},  {"c", "c", 0.5}
        };
        for (Object[] t : transitions) {
            String from = (String) t[0];
            trans.computeIfAbsent(from, k -> new ArrayList<>())
                 .add(new Object[]{t[1], t[2]});
        }

        Random rng = new Random(42);
        String state = "a";
        Map<String, Integer> counts = new LinkedHashMap<>();
        counts.put("a", 0); counts.put("b", 0); counts.put("c", 0);
        final int numSteps = 5000;

        for (int i = 0; i < numSteps; i++) {
            double r = rng.nextDouble();
            double cumulative = 0.0;
            for (Object[] entry : trans.get(state)) {
                cumulative += (double) entry[1];
                if (r < cumulative) {
                    state = (String) entry[0];
                    break;
                }
            }
            counts.put(state, counts.get(state) + 1);
        }

        System.out.printf("{ 'a': %d, 'b': %d, 'c': %d }%n",
            counts.get("a"), counts.get("b"), counts.get("c"));
    }
}
