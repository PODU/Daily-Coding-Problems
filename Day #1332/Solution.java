// Day 1332: Round each x_i up or down so sum(Y)=round(sum X) while minimizing sum|x_i-y_i|.
// Floor everything, then round up the k elements with the largest fractional parts (k=target-sum of floors). O(n log n).
import java.util.*;

public class Solution {
    static long[] roundPreserveSum(double[] x) {
        int n = x.length;
        double s = 0;
        for (double v : x) s += v;
        long target = Math.round(s);
        long[] y = new long[n];
        long base = 0;
        Integer[] idx = new Integer[n];
        double[] frac = new double[n];
        for (int i = 0; i < n; i++) {
            long f = (long) Math.floor(x[i]);
            y[i] = f; base += f;
            frac[i] = x[i] - f;
            idx[i] = i;
        }
        long k = target - base;
        Arrays.sort(idx, (a, b) -> Double.compare(frac[b], frac[a]));
        for (int i = 0; i < k; i++) y[idx[i]]++;
        return y;
    }

    public static void main(String[] args) {
        double[] x = {1.3, 2.3, 4.4};
        System.out.println(Arrays.toString(roundPreserveSum(x))); // [1, 2, 5]
    }
}
