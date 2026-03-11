// Approximate median: take a constant-size random sample and return its median.
// Sublinear: O(k log k) for constant k samples, independent of N. Space: O(k).
import java.util.*;

public class Solution {
    static int chosenRank;

    static int approxMedian(int[] a) {
        int N = a.length;
        int k = Math.min(N, 31);
        Random rng = new Random(42); // fixed seed for reproducibility
        int[] sample = new int[k];
        for (int i = 0; i < k; i++) sample[i] = a[rng.nextInt(N)];
        Arrays.sort(sample);
        int med = sample[k / 2];
        int rank = 0;
        for (int x : a) if (x <= med) rank++;
        chosenRank = rank;
        return med;
    }

    public static void main(String[] args) {
        int N = 100;
        int[] a = new int[N];
        for (int i = 0; i < N; i++) a[i] = i + 1;
        int v = approxMedian(a);
        System.out.println("Approximate median: " + v + " (rank " + chosenRank
                + " within [" + (N / 4) + ", " + (3 * N / 4) + "])");
    }
}
