// Pyramid min cost (only lowering). For each index the max pyramid height is
// min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
// retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
public class Solution {
    static long minCost(int[] a) {
        int n = a.length;
        long[] left = new long[n], right = new long[n];
        left[0] = Math.min(a[0], 1);
        for (int i = 1; i < n; i++) left[i] = Math.min(a[i], left[i-1] + 1);
        right[n-1] = Math.min(a[n-1], 1);
        for (int i = n-2; i >= 0; i--) right[i] = Math.min(a[i], right[i+1] + 1);
        long bestPeak = 0, sum = 0;
        for (int i = 0; i < n; i++) {
            bestPeak = Math.max(bestPeak, Math.min(left[i], right[i]));
            sum += a[i];
        }
        return sum - bestPeak * bestPeak;
    }

    public static void main(String[] args) {
        int[] a = {1, 1, 3, 3, 2, 1};
        System.out.println(minCost(a));
    }
}
