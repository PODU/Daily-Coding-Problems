// Reservoir sampling (k=1): keep current pick with prob 1/i as i-th item streams.
// Time: O(n) single pass, Space: O(1).
import java.util.Iterator;
import java.util.Random;
import java.util.Arrays;
import java.util.List;
import java.util.TreeMap;
import java.util.Map;

public class Solution {
    static int pickRandom(Iterator<Integer> stream, Random rng) {
        int chosen = 0, count = 0;
        while (stream.hasNext()) {
            int x = stream.next();
            count++;
            if (rng.nextInt(count) == 0) chosen = x;
        }
        return chosen;
    }

    public static void main(String[] args) {
        List<Integer> stream = Arrays.asList(10, 20, 30, 40, 50);
        Random rng = new Random(42);
        Map<Integer, Integer> freq = new TreeMap<>();
        for (int t = 0; t < 100000; t++) {
            int s = pickRandom(stream.iterator(), rng);
            freq.merge(s, 1, Integer::sum);
        }
        System.out.println("One sample: " + pickRandom(stream.iterator(), rng));
        System.out.println("Approx frequencies over 100000 trials:");
        for (Map.Entry<Integer, Integer> e : freq.entrySet())
            System.out.printf("  %d: %.3f%n", e.getKey(), e.getValue() / 100000.0);
    }
}
