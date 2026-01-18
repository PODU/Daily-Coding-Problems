// Floor all; round up the `deficit` elements with largest fractional parts to match round(sum). O(n log n) time, O(n) space.
import java.util.Arrays;
import java.util.Comparator;

public class Solution {
    static int[] roundToSum(double[] x) {
        int n = x.length;
        int[] y = new int[n];
        double sum = 0;
        long floorSum = 0;
        Integer[] idx = new Integer[n];
        double[] frac = new double[n];
        for (int i = 0; i < n; i++) {
            double f = Math.floor(x[i]);
            y[i] = (int) f;
            floorSum += (long) f;
            sum += x[i];
            frac[i] = x[i] - f;
            idx[i] = i;
        }
        long target = Math.round(sum);
        long deficit = target - floorSum;
        Arrays.sort(idx, Comparator.comparingDouble((Integer i) -> frac[i]).reversed());
        for (long k = 0; k < deficit && k < n; k++) y[idx[(int) k]]++;
        return y;
    }

    public static void main(String[] args) {
        double[] x = {1.3, 2.3, 4.4};
        int[] y = roundToSum(x);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < y.length; i++) {
            sb.append(y[i]);
            if (i + 1 < y.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
