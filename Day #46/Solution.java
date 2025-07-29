// Day 46: Longest palindromic substring via Manacher's algorithm.
// Time: O(n), Space: O(n).
public class Solution {
    static String longestPalindrome(String s) {
        if (s.isEmpty()) return "";
        StringBuilder sb = new StringBuilder("^");
        for (char c : s.toCharArray()) sb.append('#').append(c);
        sb.append("#$");
        String t = sb.toString();
        int n = t.length();
        int[] p = new int[n];
        int c = 0, r = 0;
        for (int i = 1; i < n - 1; i++) {
            if (i < r) p[i] = Math.min(r - i, p[2 * c - i]);
            while (t.charAt(i + 1 + p[i]) == t.charAt(i - 1 - p[i])) p[i]++;
            if (i + p[i] > r) { c = i; r = i + p[i]; }
        }
        int maxLen = 0, centerIndex = 0;
        for (int i = 1; i < n - 1; i++)
            if (p[i] > maxLen) { maxLen = p[i]; centerIndex = i; }
        int start = (centerIndex - maxLen) / 2;
        return s.substring(start, start + maxLen);
    }

    public static void main(String[] args) {
        System.out.println("\"" + longestPalindrome("aabcdcb") + "\"");
        System.out.println("\"" + longestPalindrome("bananas") + "\"");
    }
}
