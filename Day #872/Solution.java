// Approximate median: take a small random sample (size s) and return its median.
// Rank lands in [N/4, 3N/4] w.h.p. Time O(s log s) = sub-linear, Space O(s).
import java.util.*;

public class Solution {
    static int approxMedian(int[] a, int sampleSize, Random rng){
        int n = a.length;
        int s = Math.min(sampleSize, n);
        int[] sample = new int[s];
        for(int i = 0; i < s; i++) sample[i] = a[rng.nextInt(n)];
        Arrays.sort(sample);
        return sample[s / 2];
    }

    public static void main(String[] args){
        Random rng = new Random(42);
        int n = 1000;
        Integer[] boxed = new Integer[n];
        for(int i = 0; i < n; i++) boxed[i] = i;
        Collections.shuffle(Arrays.asList(boxed), rng);
        int[] a = new int[n];
        for(int i = 0; i < n; i++) a[i] = boxed[i];

        int m = approxMedian(a, 51, rng);
        int rank = 0;
        for(int x : a) if(x < m) rank++;
        System.out.println("approx median = " + m);
        boolean ok = rank >= n / 4 && rank <= 3 * n / 4;
        System.out.println("rank " + rank + " in [" + n/4 + ", " + 3*n/4 + "]: " + ok);
    }
}
