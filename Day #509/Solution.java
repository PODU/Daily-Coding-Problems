// Day 509: Fewest-insertion palindrome with lexicographically earliest result.
// Memoized interval DP building the actual string. Time O(n^3), Space O(n^2).
public class Solution {
    static String s;
    static String[][] memo;

    static String build(int i, int j) {
        if (i > j) return "";
        if (i == j) return String.valueOf(s.charAt(i));
        if (memo[i][j] != null) return memo[i][j];
        String res;
        if (s.charAt(i) == s.charAt(j)) {
            res = s.charAt(i) + build(i + 1, j - 1) + s.charAt(j);
        } else {
            String a = s.charAt(i) + build(i + 1, j) + s.charAt(i);
            String b = s.charAt(j) + build(i, j - 1) + s.charAt(j);
            if (a.length() < b.length()) res = a;
            else if (b.length() < a.length()) res = b;
            else res = (a.compareTo(b) <= 0) ? a : b;
        }
        return memo[i][j] = res;
    }

    static String solve(String in) {
        s = in;
        int n = s.length();
        if (n == 0) return "";
        memo = new String[n][n];
        return build(0, n - 1);
    }

    public static void main(String[] args) {
        System.out.println(solve("race"));
        System.out.println(solve("google"));
    }
}
