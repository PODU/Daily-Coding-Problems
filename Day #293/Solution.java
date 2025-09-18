// Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
// Time O(n), Space O(n).
public class Solution {
    static long minCost(int[] h) {
        int n = h.length;
        long[] L = new long[n], R = new long[n];
        L[0] = Math.min(h[0], 1);
        for (int i = 1; i < n; i++) L[i] = Math.min(h[i], L[i-1] + 1);
        R[n-1] = Math.min(h[n-1], 1);
        for (int i = n-2; i >= 0; i--) R[i] = Math.min(h[i], R[i+1] + 1);
        long v = 0, sum = 0;
        for (int i = 0; i < n; i++) { v = Math.max(v, Math.min(L[i], R[i])); sum += h[i]; }
        return sum - v * v;
    }

    public static void main(String[] args) {
        int[] h = {1, 1, 3, 3, 2, 1};
        System.out.println(minCost(h));
    }
}
