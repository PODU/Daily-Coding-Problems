// Round floats to ints keeping sum == round(total) with minimal total abs diff.
// Floor all; round up (T-F) elements with largest fractional parts (cheapest to push up). O(N log N) time, O(N) space.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        double[] x = {1.3, 2.3, 4.4};
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
        long need = target - floorSum;

        Integer[] idx = new Integer[n];
        for (int i = 0; i < n; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> Double.compare(
            x[b] - Math.floor(x[b]), x[a] - Math.floor(x[a])));

        for (int k = 0; k < n && need > 0; k++) {
            int i = idx[k];
            if (x[i] - Math.floor(x[i]) > 0) { y[i] += 1; need--; }
        }

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < n; i++) {
            sb.append(y[i]);
            if (i + 1 < n) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
