// Subset Sum: boolean DP over reachable sums; reconstruct one subset by backtracking.
// Time O(n*k), Space O(n*k).
import java.util.*;

public class Solution {
    static List<Integer> subsetSum(int[] S, int k) {
        int n = S.length;
        boolean[][] reach = new boolean[n + 1][k + 1];
        reach[0][0] = true;
        for (int i = 1; i <= n; i++) {
            for (int s = 0; s <= k; s++) {
                if (reach[i - 1][s]) reach[i][s] = true;
                if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) reach[i][s] = true;
            }
        }
        if (!reach[n][k]) return null;
        List<Integer> chosen = new ArrayList<>();
        int s = k;
        for (int i = n; i >= 1; i--) {
            if (s >= S[i - 1] && reach[i - 1][s - S[i - 1]]) {
                chosen.add(S[i - 1]);
                s -= S[i - 1];
            }
        }
        return chosen;
    }

    public static void main(String[] args) {
        int[] S = {12, 1, 61, 5, 9, 2};
        int k = 24;
        List<Integer> sub = subsetSum(S, k);
        System.out.println(sub);
        int total = 0;
        if (sub != null) for (int x : sub) total += x;
        System.out.println("Sum = " + total);
    }
}
