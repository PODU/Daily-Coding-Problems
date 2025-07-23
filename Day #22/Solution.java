// Word Break reconstruction: DP over positions with memoization using a word set.
// Time: O(n^2) substring checks (n = string length), Space: O(n) for memo + recursion.
import java.util.*;

public class Solution {
    static int[] memo;

    static boolean solve(String s, Set<String> dict, int i) {
        int n = s.length();
        if (i == n) return true;
        if (memo[i] != -2) return memo[i] != -1;
        for (int j = i + 1; j <= n; j++) {
            if (dict.contains(s.substring(i, j)) && solve(s, dict, j)) {
                memo[i] = j - i;
                return true;
            }
        }
        memo[i] = -1;
        return false;
    }

    static List<String> wordBreak(String s, Set<String> dict) {
        int n = s.length();
        memo = new int[n + 1];
        Arrays.fill(memo, -2);
        if (!solve(s, dict, 0)) return null;
        List<String> res = new ArrayList<>();
        for (int i = 0; i < n; ) {
            res.add(s.substring(i, i + memo[i]));
            i += memo[i];
        }
        return res;
    }

    public static void main(String[] args) {
        Set<String> dict = new HashSet<>(Arrays.asList("quick", "brown", "the", "fox"));
        String s = "thequickbrownfox";
        List<String> res = wordBreak(s, dict);
        if (res == null) {
            System.out.println("None");
            return;
        }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
