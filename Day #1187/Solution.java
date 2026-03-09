// Markov chain simulation: cumulative transition table, draw uniform RNG per step.
// Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Object[][] transitions = {
            {'a','a',0.9},{'a','b',0.075},{'a','c',0.025},
            {'b','a',0.15},{'b','b',0.8},{'b','c',0.05},
            {'c','a',0.25},{'c','b',0.25},{'c','c',0.5}
        };
        Map<Character, List<double[]>> raw = new TreeMap<>();
        Map<Character, List<Object[]>> labels = new TreeMap<>();
        // build cumulative probabilities per state: store {cumProb, nextState}
        Map<Character, List<Object[]>> table = new TreeMap<>();
        for (Object[] t : transitions) {
            char from = (Character) t[0];
            table.computeIfAbsent(from, k -> new ArrayList<>());
        }
        for (Object[] t : transitions) {
            char from = (Character) t[0];
            char to = (Character) t[1];
            double p = (Double) t[2];
            table.get(from).add(new Object[]{to, p});
        }
        Map<Character, List<Object[]>> cumTable = new TreeMap<>();
        for (Map.Entry<Character, List<Object[]>> e : table.entrySet()) {
            double cum = 0.0;
            List<Object[]> lst = new ArrayList<>();
            for (Object[] entry : e.getValue()) {
                cum += (Double) entry[1];
                lst.add(new Object[]{(Character) entry[0], cum});
            }
            cumTable.put(e.getKey(), lst);
        }

        char start = 'a';
        int numSteps = 5000;
        Map<Character, Long> counts = new TreeMap<>();
        counts.put('a', 0L); counts.put('b', 0L); counts.put('c', 0L);

        Random rng = new Random(12345);
        char state = start;
        for (int i = 0; i < numSteps; i++) {
            double r = rng.nextDouble();
            for (Object[] entry : cumTable.get(state)) {
                if (r < (Double) entry[1]) { state = (Character) entry[0]; break; }
            }
            counts.put(state, counts.get(state) + 1);
        }

        System.out.println("{'a': " + counts.get('a') + ", 'b': " + counts.get('b') + ", 'c': " + counts.get('c') + "}");
    }
}
