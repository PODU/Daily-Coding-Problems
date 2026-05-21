// Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
// unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
import java.util.*;

public class Solution {
    static double estimatePi(long samples, long seed) {
        Random rng = new Random(seed);
        long inside = 0;
        for (long i = 0; i < samples; i++) {
            double x = rng.nextDouble(), y = rng.nextDouble();
            if (x * x + y * y <= 1.0) inside++;
        }
        return 4.0 * inside / samples;
    }

    public static void main(String[] args) {
        double pi = estimatePi(10_000_000L, 42L);
        System.out.printf("%.3f%n", pi);
    }
}
