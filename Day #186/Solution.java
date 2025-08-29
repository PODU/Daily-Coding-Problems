// Day 186: Minimum subset-sum difference (partition problem).
// Subset-sum DP over total, pick best <= total/2, reconstruct. Time O(n*S), Space O(n*S).
import java.util.*;

public class Solution {
    static void solve(int[] a) {
        int n = a.length, tot = 0;
        for (int x : a) tot += x;
        boolean[][] dp = new boolean[n + 1][tot + 1];
        dp[0][0] = true;
        for (int i = 1; i <= n; i++)
            for (int s = 0; s <= tot; s++)
                dp[i][s] = dp[i - 1][s] || (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]);
        int best = 0;
        for (int s = tot / 2; s >= 0; s--) if (dp[n][s]) { best = s; break; }
        List<Integer> sub = new ArrayList<>(), other = new ArrayList<>();
        int s = best;
        for (int i = n; i >= 1; i--) {
            if (s >= a[i - 1] && dp[i - 1][s - a[i - 1]]) { sub.add(a[i - 1]); s -= a[i - 1]; }
            else other.add(a[i - 1]);
        }
        Collections.reverse(sub); Collections.reverse(other);
        System.out.println(fmt(sub) + " and " + fmt(other));
    }

    static String fmt(List<Integer> v) {
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < v.size(); i++) { sb.append(v.get(i)); if (i + 1 < v.size()) sb.append(", "); }
        return sb.append("}").toString();
    }

    public static void main(String[] args) {
        solve(new int[]{5, 10, 15, 20, 25});
    }
}
