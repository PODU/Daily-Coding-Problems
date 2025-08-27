// Day 181: Minimum palindrome partitioning.
// DP: palindrome table + min-cut DP with reconstruction. Time O(n^2), Space O(n^2).
import java.util.*;

public class Solution {
    static List<String> minPalindromePartition(String s) {
        int n = s.length();
        List<String> res = new ArrayList<>();
        if (n == 0) return res;
        boolean[][] pal = new boolean[n][n];
        for (int i = 0; i < n; i++) pal[i][i] = true;
        for (int L = 2; L <= n; L++)
            for (int i = 0; i + L - 1 < n; i++) {
                int j = i + L - 1;
                if (s.charAt(i) == s.charAt(j) && (L == 2 || pal[i + 1][j - 1])) pal[i][j] = true;
            }
        int[] cut = new int[n + 1], prev = new int[n + 1];
        Arrays.fill(cut, Integer.MAX_VALUE);
        Arrays.fill(prev, -1);
        cut[0] = 0;
        for (int i = 1; i <= n; i++)
            for (int j = 0; j < i; j++)
                if (pal[j][i - 1] && cut[j] != Integer.MAX_VALUE && cut[j] + 1 < cut[i]) {
                    cut[i] = cut[j] + 1; prev[i] = j;
                }
        for (int i = n; i > 0; i = prev[i]) res.add(s.substring(prev[i], i));
        Collections.reverse(res);
        return res;
    }

    static void printRes(List<String> v) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) {
            sb.append("\"").append(v.get(i)).append("\"");
            if (i + 1 < v.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        printRes(minPalindromePartition("racecarannakayak"));
        printRes(minPalindromePartition("abc"));
    }
}
