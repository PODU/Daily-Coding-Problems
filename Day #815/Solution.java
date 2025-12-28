// Monte Carlo pi: sample (x,y) in unit square via deterministic splitmix64 RNG,
// pi ~= 4*inside/total. Fixed seed -> deterministic. Time O(N), Space O(1).
public class Solution {
    static long state;

    static long nextU64() {
        state += 0x9E3779B97F4A7C15L;
        long z = state;
        z = (z ^ (z >>> 30)) * 0xBF58476D1CE4E5B9L;
        z = (z ^ (z >>> 27)) * 0x94D049BB133111EBL;
        return z ^ (z >>> 31);
    }

    static double nextDouble() {
        return (nextU64() >>> 11) * (1.0 / 9007199254740992.0);
    }

    public static void main(String[] args) {
        state = 42L;
        final long N = 10000000L;
        long inside = 0;
        for (long i = 0; i < N; i++) {
            double x = nextDouble(), y = nextDouble();
            if (x * x + y * y <= 1.0) inside++;
        }
        double pi = 4.0 * inside / N;
        System.out.printf("Estimated pi ≈ %.3f%n", pi);
    }
}
