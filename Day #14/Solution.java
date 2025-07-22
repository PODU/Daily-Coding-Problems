// Estimate pi via Monte Carlo: fraction of random points in unit circle * 4.
// Time: O(samples), Space: O(1). Seeded for reproducible 3-decimal output.
import java.util.Random;

public class Solution {
    static double estimatePi(long samples) {
        Random rng = new Random(12345);
        long inside = 0;
        for (long i = 0; i < samples; i++) {
            double x = rng.nextDouble(), y = rng.nextDouble();
            if (x * x + y * y <= 1.0) inside++;
        }
        return 4.0 * inside / samples;
    }

    public static void main(String[] args) {
        System.out.printf("%.3f%n", estimatePi(10000000L));
    }
}
