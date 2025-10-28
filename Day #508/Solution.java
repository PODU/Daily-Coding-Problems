// Approximate median: sample a fixed, sublinear subset (size independent of N),
// return the sample's median -> lands in the central half [N/4, 3N/4] w.h.p.
// Sampling+median: O(s log s), sublinear in N. (Rank shown only for the demo.)
import java.util.Arrays;

public class Solution {
    static long state = 0x0123456789ABCDEFL; // fixed seed -> deterministic

    static long nextRand() {
        state = state * 6364136223846793005L + 1442695040888963407L; // wraps mod 2^64
        return state >>> 33; // top 31 bits, non-negative
    }

    public static void main(String[] args) {
        final int N = 1000;
        final int SAMPLE_SIZE = 99; // fixed, independent of N (sublinear)

        int[] data = new int[N];
        for (int i = 0; i < N; i++) data[i] = i + 1;
        for (int i = N - 1; i > 0; i--) {
            int j = (int) (nextRand() % (i + 1));
            int t = data[i]; data[i] = data[j]; data[j] = t;
        }

        int[] sample = new int[SAMPLE_SIZE];
        for (int i = 0; i < SAMPLE_SIZE; i++) {
            int idx = (int) (nextRand() % N);
            sample[i] = data[idx];
        }
        Arrays.sort(sample);
        int median = sample[SAMPLE_SIZE / 2];

        int rank = 0;
        for (int v : data) if (v <= median) rank++;

        System.out.println("Approximate median: " + median);
        System.out.println("Rank: " + rank + " (acceptable range: " + (N / 4)
                + " to " + (3 * N / 4) + ")");
    }
}
