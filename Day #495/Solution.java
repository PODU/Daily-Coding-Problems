// Day 495: Reservoir sampling (size 1) from a stream of unknown length.
// For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
// Time: O(n) per pass, Space: O(1).
import java.util.*;

public class Solution {
    // Processes a stream via an iterator without storing it.
    static int reservoirSample(Iterator<Integer> stream, Random rng) {
        int chosen = 0, count = 0;
        while (stream.hasNext()) {
            int x = stream.next();
            count++;
            if (rng.nextDouble() < 1.0 / count) chosen = x;
        }
        return chosen;
    }

    public static void main(String[] args) {
        List<Integer> stream = new ArrayList<>();
        for (int i = 1; i <= 10; i++) stream.add(i);

        Random rng = new Random(42);
        int TRIALS = 100000;
        Map<Integer, Integer> counts = new HashMap<>();
        for (int t = 0; t < TRIALS; t++) {
            int v = reservoirSample(stream.iterator(), rng);
            counts.merge(v, 1, Integer::sum);
        }

        System.out.println("Empirical selection frequency per element (~0.100 each):");
        for (int v : stream)
            System.out.printf("%d: %.3f%n", v, counts.getOrDefault(v, 0) / (double) TRIALS);
        System.out.println("One sampled value: " + reservoirSample(stream.iterator(), rng));
    }
}
