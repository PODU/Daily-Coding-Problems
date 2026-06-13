// Shortest palindrome by insertions, lexicographically earliest: memoized DP on
// (i,j) building best palindrome for s[i..j]. Time O(n^2) states, Space O(n^2).
public class Solution {
    static String s;
    static String[][] memo;

    static String solve(int i, int j) {
        if (i > j) return "";
        if (i == j) return String.valueOf(s.charAt(i));
        if (memo[i][j] != null) return memo[i][j];
        String res;
        if (s.charAt(i) == s.charAt(j)) {
            res = s.charAt(i) + solve(i + 1, j - 1) + s.charAt(i);
        } else {
            String opt1 = s.charAt(i) + solve(i + 1, j) + s.charAt(i);
            String opt2 = s.charAt(j) + solve(i, j - 1) + s.charAt(j);
            if (opt1.length() < opt2.length()) res = opt1;
            else if (opt2.length() < opt1.length()) res = opt2;
            else res = (opt1.compareTo(opt2) <= 0) ? opt1 : opt2;
        }
        memo[i][j] = res;
        return res;
    }

    static String shortestPalindrome(String input) {
        s = input;
        int n = s.length();
        memo = new String[n][n];
        return solve(0, n - 1);
    }

    public static void main(String[] args) {
        System.out.println(shortestPalindrome("race"));
        System.out.println(shortestPalindrome("google"));
    }
}
