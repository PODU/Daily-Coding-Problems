// Monte Carlo pi estimate with a shared 64-bit LCG. Time O(n), Space O(1).
public class Solution {
    static final long A = 6364136223846793005L;
    static final long C = 1442695040888963407L;

    static double estimatePi(long samples, long seed) {
        long x = seed;
        long inside = 0;
        for (long i = 0; i < samples; i++) {
            x = A * x + C;
            double px = (double) (x >>> 11) / (double) (1L << 53);
            x = A * x + C;
            double py = (double) (x >>> 11) / (double) (1L << 53);
            if (px * px + py * py <= 1.0) inside++;
        }
        return 4.0 * inside / samples;
    }

    public static void main(String[] args) {
        System.out.printf("%.3f%n", estimatePi(2000000, 42));
    }
}
