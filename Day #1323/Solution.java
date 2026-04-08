// Day 1323: Min lowering cost to form a pyramid (rise by 1 to a peak, fall by 1, unit ends).
// left[i]=min(a[i],left[i-1]+1), right[i] symmetric; best peak h=max(min(left,right)); pyramid sums to h^2.
// Cost = sum(a) - h^2. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] a = {1, 1, 3, 3, 2, 1};
        int n = a.length;
        int[] left = new int[n], right = new int[n];
        for (int i = 0; i < n; i++) left[i] = Math.min(a[i], (i > 0 ? left[i-1] : 0) + 1);
        for (int i = n - 1; i >= 0; i--) right[i] = Math.min(a[i], (i < n-1 ? right[i+1] : 0) + 1);

        int h = 0, peak = 0;
        for (int i = 0; i < n; i++) { int hi = Math.min(left[i], right[i]); if (hi > h) { h = hi; peak = i; } }

        int[] target = new int[n];
        for (int i = 0; i < n; i++) { int d = Math.abs(i - peak); if (d < h) target[i] = h - d; }
        long total = 0; for (int v : a) total += v;
        long cost = total - (long) h * h;

        System.out.println(cost);                    // 2
        System.out.println(Arrays.toString(target)); // [0, 1, 2, 3, 2, 1]
    }
}
