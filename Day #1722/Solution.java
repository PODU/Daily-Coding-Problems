// Day 1722: Approximate median via random sampling.
// Sample a sublinear number of elements (~constant), return their exact median.
// With high probability its rank lies in [N/4, 3N/4]. Time: O(s log s) << O(N), Space: O(s).
import java.util.*;

public class Solution {
    static int approxMedian(int[] a, Random rng) {
        int n = a.length;
        int s = Math.min(n, 99); // sublinear sample size
        int[] sample = new int[s];
        for (int i = 0; i < s; i++) sample[i] = a[rng.nextInt(n)];
        Arrays.sort(sample);
        return sample[s / 2];
    }

    public static void main(String[] args) {
        // Demo: values 0..99 shuffled deterministically.
        int N = 100;
        Integer[] boxed = new Integer[N];
        for (int i = 0; i < N; i++) boxed[i] = i;
        Random rng = new Random(42);
        Collections.shuffle(Arrays.asList(boxed), rng);
        int[] a = new int[N];
        for (int i = 0; i < N; i++) a[i] = boxed[i];

        int m = approxMedian(a, rng);
        int rank = 0;
        for (int x : a) if (x < m) rank++;
        System.out.println("Approximate median: " + m
            + " (rank " + rank + " within [N/4, 3N/4] = ["
            + (N / 4) + ", " + (3 * N / 4) + "])");
    }
}
