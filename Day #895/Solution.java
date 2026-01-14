// Palindrome by deleting at most k chars: min deletions = n - LPS(s).
// LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
public class Solution {
    static boolean canMakePalindrome(String s, int k) {
        int n = s.length();
        int[] prev = new int[n], cur = new int[n];
        for (int i = n - 1; i >= 0; --i) {
            cur = new int[n];
            cur[i] = 1;
            for (int j = i + 1; j < n; ++j) {
                if (s.charAt(i) == s.charAt(j)) cur[j] = prev[j - 1] + 2;
                else cur[j] = Math.max(prev[j], cur[j - 1]);
            }
            prev = cur;
        }
        int lps = n == 0 ? 0 : cur[n - 1];
        return (n - lps) <= k;
    }

    public static void main(String[] args) {
        System.out.println(canMakePalindrome("waterrfetawx", 2) ? "True" : "False");
    }
}
