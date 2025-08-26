// Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
// Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
import java.util.*;

public class Solution {
    public static Map<Character, Integer> simulate(char start, int numSteps,
            Object[][] transitions, long seed) {
        Map<Character, List<double[]>> trans = new HashMap<>();
        Map<Character, Character> dstChar = new HashMap<>();
        // store dst as char in a parallel structure via index list
        Map<Character, List<Object[]>> t = new HashMap<>();
        for (Object[] tr : transitions) {
            char src = (Character) tr[0];
            t.computeIfAbsent(src, k -> new ArrayList<>()).add(tr);
        }
        Random rng = new Random(seed);
        Map<Character, Integer> counts = new HashMap<>();
        char state = start;
        for (int i = 0; i < numSteps; i++) {
            counts.merge(state, 1, Integer::sum);
            double r = rng.nextDouble();
            double cum = 0.0;
            for (Object[] tr : t.get(state)) {
                cum += (Double) tr[2];
                if (r < cum) { state = (Character) tr[1]; break; }
            }
        }
        return counts;
    }

    public static void main(String[] args) {
        Object[][] transitions = {
            {'a','a',0.9},{'a','b',0.075},{'a','c',0.025},
            {'b','a',0.15},{'b','b',0.8},{'b','c',0.05},
            {'c','a',0.25},{'c','b',0.25},{'c','c',0.5},
        };
        Map<Character, Integer> c = simulate('a', 5000, transitions, 42L);
        System.out.println("{'a': " + c.getOrDefault('a', 0) + ", 'b': "
            + c.getOrDefault('b', 0) + ", 'c': " + c.getOrDefault('c', 0) + "}");
    }
}
