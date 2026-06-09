// Min subset-sum difference: 0/1 subset-sum DP over reachable sums, pick best<=total/2; backtrack subset. O(n*total) time/space.
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Solution {
    public static void main(String[] args) {
        int[] a = {5, 10, 15, 20, 25};
        int n = a.length;
        int total = 0;
        for (int x : a) total += x;

        // dp[i][s] = sum s reachable using first i items
        boolean[][] dp = new boolean[n + 1][total + 1];
        dp[0][0] = true;
        for (int i = 1; i <= n; i++)
            for (int s = 0; s <= total; s++) {
                dp[i][s] = dp[i - 1][s];
                if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) dp[i][s] = true;
            }

        int best = 0;
        for (int s = total / 2; s >= 0; s--)
            if (dp[n][s]) { best = s; break; }

        // Backtrack from last item to first to recover subset A
        List<Integer> A = new ArrayList<>();
        boolean[] used = new boolean[n];
        int s = best;
        for (int i = n; i >= 1; i--) {
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) {
                A.add(a[i - 1]);
                used[i - 1] = true;
                s -= a[i - 1];
            }
        }
        Collections.sort(A);

        List<Integer> B = new ArrayList<>();
        for (int i = 0; i < n; i++)
            if (!used[i]) B.add(a[i]);

        System.out.println("Minimum difference: " + (total - 2 * best));
        System.out.println("Subset A: " + format(A));
        System.out.println("Subset B: " + format(B));
    }

    static String format(List<Integer> v) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(v.get(i));
        }
        return sb.append("]").toString();
    }
}
