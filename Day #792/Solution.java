// Pyramid: left[i]=min(h,left[i-1]+1), right[i]=min(h,right[i+1]+1), cap=min(left,right).
// Peak P=max(cap); target descends from P both sides. cost=sum(h)-sum(target). O(n) time/space.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] h = {1, 1, 3, 3, 2, 1};
        int n = h.length;
        int[] left = new int[n], right = new int[n];
        left[0] = Math.min(h[0], 1);
        for (int i = 1; i < n; i++) left[i] = Math.min(h[i], left[i-1] + 1);
        right[n-1] = Math.min(h[n-1], 1);
        for (int i = n - 2; i >= 0; i--) right[i] = Math.min(h[i], right[i+1] + 1);
        int[] cap = new int[n];
        int P = 0, k = 0;
        for (int i = 0; i < n; i++) {
            cap[i] = Math.min(left[i], right[i]);
            if (cap[i] > P) { P = cap[i]; k = i; }
        }
        int[] target = new int[n];
        target[k] = P;
        for (int j = 1; k - j >= 0; j++) target[k-j] = Math.max(0, P - j);
        for (int j = 1; k + j < n; j++) target[k+j] = Math.max(0, P - j);
        int cost = 0;
        for (int i = 0; i < n; i++) cost += h[i] - target[i];
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append(target[i]);
            if (i + 1 < n) sb.append(", ");
        }
        System.out.println(cost + " (resulting in [" + sb + "])");
    }
}
