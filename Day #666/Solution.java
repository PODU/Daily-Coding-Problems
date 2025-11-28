// Day 666: Minimize difference of two subset sums. Subset-sum DP over reachable sums up to total/2,
// pick the reachable sum closest to total/2, reconstruct one subset. Time O(n*sum), Space O(n*sum).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] a = {5, 10, 15, 20, 25};
        int n = a.length, total = 0;
        for (int x : a) total += x;
        int half = total / 2;
        boolean[][] dp = new boolean[n + 1][half + 1];
        for (int i = 0; i <= n; i++) dp[i][0] = true;
        for (int i = 1; i <= n; i++)
            for (int s = 0; s <= half; s++)
                dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
        int best = 0;
        for (int s = half; s >= 0; s--) if (dp[n][s]) { best = s; break; }
        List<Integer> A = new ArrayList<>(), B = new ArrayList<>();
        int s = best;
        for (int i = n; i >= 1; i--) {
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { A.add(a[i - 1]); s -= a[i - 1]; }
            else B.add(a[i - 1]);
        }
        System.out.println(fmt(A) + " and " + fmt(B) + ", difference of " + (total - 2 * best));
    }

    static String fmt(List<Integer> l) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < l.size(); i++) sb.append(l.get(i)).append(i + 1 < l.size() ? ", " : "");
        return sb.append("}").toString();
    }
}
