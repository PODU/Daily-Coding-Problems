// Round floats preserving sum: floor all, then round up the d elements with
// largest fractional parts (d = round(sum) - sum(floors)). Time O(n log n).
import java.util.*;

public class Solution {
    static long[] roundPreserve(double[] x) {
        int n = x.length;
        long[] y = new long[n];
        double sum = 0;
        long floorSum = 0;
        for (int i = 0; i < n; i++) {
            long f = (long) Math.floor(x[i]);
            y[i] = f;
            floorSum += f;
            sum += x[i];
        }
        long target = Math.round(sum);
        long d = target - floorSum;
        Integer[] order = new Integer[n];
        for (int i = 0; i < n; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Double.compare(
                (x[b] - Math.floor(x[b])), (x[a] - Math.floor(x[a]))));
        for (int i = 0; i < d; i++) y[order[i]] += 1;
        return y;
    }

    public static void main(String[] args) {
        double[] x = {1.3, 2.3, 4.4};
        long[] y = roundPreserve(x);

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < y.length; i++) { if (i > 0) sb.append(", "); sb.append(y[i]); }
        sb.append("]");
        System.out.println(sb);

        double diff = 0;
        for (int i = 0; i < x.length; i++) diff += Math.abs(x[i] - y[i]);
        System.out.printf("abs diff = %.1f%n", diff);
    }
}
