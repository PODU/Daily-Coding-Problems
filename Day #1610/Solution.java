// Reservoir sampling (reservoir size 1): for the i-th element replace kept with prob 1/i.
// Distribution is uniform over the stream. Time O(n), Space O(1). Seeded RNG -> deterministic.
import java.util.Random;

public class Solution {
    static int reservoirSample(int[] stream, long seed) {
        Random rng = new Random(seed);
        int kept = 0;
        for (int i = 0; i < stream.length; i++) {
            // for the (i+1)-th element keep with prob 1/(i+1)
            if (rng.nextInt(i + 1) == 0) kept = stream[i];
        }
        return kept;
    }

    public static void main(String[] args) {
        int[] stream = new int[10];
        for (int v = 1; v <= 10; v++) stream[v - 1] = v;
        int selected = reservoirSample(stream, 42L);
        System.out.println("Selected: " + selected);
    }
}
