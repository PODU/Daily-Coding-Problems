// Fewest insertions for palindrome; lexicographically smallest among minima.
// Interval DP with memoized reconstruction. Time/Space O(n^2).
public class Solution {
    static char[] s;
    static String[][] memo;
    static boolean[][] done;

    static String build(int i, int j) {
        if (i > j) return "";
        if (i == j) return String.valueOf(s[i]);
        if (done[i][j]) return memo[i][j];
        done[i][j] = true;
        String res;
        if (s[i] == s[j]) {
            res = s[i] + build(i + 1, j - 1) + s[i];
        } else {
            String left = s[i] + build(i + 1, j) + s[i];
            String right = s[j] + build(i, j - 1) + s[j];
            if (left.length() != right.length())
                res = left.length() < right.length() ? left : right;
            else
                res = left.compareTo(right) <= 0 ? left : right;
        }
        return memo[i][j] = res;
    }

    static String makePalindrome(String str) {
        s = str.toCharArray();
        int n = s.length;
        memo = new String[n][n];
        done = new boolean[n][n];
        return build(0, n - 1);
    }

    public static void main(String[] args) {
        System.out.println(makePalindrome("race"));
        System.out.println(makePalindrome("google"));
    }
}
