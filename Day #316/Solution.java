// Reconstruct coin denominations from a ways-to-make-change array.
// DP coin detection: A[i] > ways[i] means i is a coin. Time O(N^2), Space O(N).
import java.util.*;

public class Solution {
    static String joinCoins(List<Integer> c) {
        if (c.isEmpty()) return "";
        if (c.size() == 1) return String.valueOf(c.get(0));
        if (c.size() == 2) return c.get(0) + " and " + c.get(1);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i + 1 < c.size(); i++) sb.append(c.get(i)).append(", ");
        sb.append("and ").append(c.get(c.size() - 1));
        return sb.toString();
    }

    static List<Integer> findCoins(int[] A) {
        int n = A.length;
        long[] ways = new long[n];
        ways[0] = 1;
        List<Integer> coins = new ArrayList<>();
        for (int i = 1; i < n; i++) {
            if (A[i] > ways[i]) {
                coins.add(i);
                for (int j = i; j < n; j++) ways[j] += ways[j - i];
            }
        }
        return coins;
    }

    public static void main(String[] args) {
        int[] A = {1, 0, 1, 1, 2};
        System.out.println(joinCoins(findCoins(A)));
    }
}
