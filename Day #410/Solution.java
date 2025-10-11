// Day 410: Round floats so rounded sum is preserved with minimal total abs error.
// Floor all, then round up the R = round(sum)-sum(floors) elements with largest fractions. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static long[] roundToSum(double[] x) {
        int n = x.length;
        long[] y = new long[n];
        double total = 0;
        long floorSum = 0;
        for (int i = 0; i < n; i++) {
            y[i] = (long) Math.floor(x[i]);
            floorSum += y[i];
            total += x[i];
        }
        long target = Math.round(total);
        long R = target - floorSum;
        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> Double.compare(
            (x[b] - Math.floor(x[b])), (x[a] - Math.floor(x[a]))));
        for (long i = 0; i < R && i < n; i++) y[idx[(int) i]] += 1;
        return y;
    }

    public static void main(String[] args) {
        double[] x = {1.3, 2.3, 4.4};
        long[] y = roundToSum(x);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < y.length; i++) {
            sb.append(y[i]);
            if (i + 1 < y.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
