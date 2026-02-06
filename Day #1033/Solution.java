// Day 1033: Minimum subset-sum difference (partition into two subsets).
// Boolean subset-sum DP over reachable sums up to total/2; answer total-2*best. O(n*sum) time, O(sum) space.
public class Solution {
    static int minDiff(int[] a) {
        int total = 0;
        for (int x : a) total += x;
        boolean[] dp = new boolean[total / 2 + 1];
        dp[0] = true;
        for (int x : a)
            for (int s = total / 2; s >= x; s--)
                if (dp[s - x]) dp[s] = true;
        for (int s = total / 2; s >= 0; s--)
            if (dp[s]) return total - 2 * s;
        return total;
    }

    public static void main(String[] args) {
        int[] a = {5, 10, 15, 20, 25};
        System.out.println(minDiff(a));
    }
}
