// Longest palindromic substring via Manacher's algorithm. Time O(n), space O(n).
public class Solution {
    static String longestPalindrome(String s) {
        if (s.isEmpty()) return "";
        StringBuilder sb = new StringBuilder("^");
        for (char c : s.toCharArray()) { sb.append('#').append(c); }
        sb.append("#$");
        String t = sb.toString();
        int n = t.length();
        int[] p = new int[n];
        int center = 0, right = 0;
        for (int i = 1; i < n - 1; i++) {
            int mirror = 2 * center - i;
            if (i < right) p[i] = Math.min(right - i, p[mirror]);
            while (t.charAt(i + p[i] + 1) == t.charAt(i - p[i] - 1)) p[i]++;
            if (i + p[i] > right) { center = i; right = i + p[i]; }
        }
        int maxLen = 0, centerIndex = 0;
        for (int i = 1; i < n - 1; i++) {
            if (p[i] > maxLen) { maxLen = p[i]; centerIndex = i; }
        }
        int start = (centerIndex - maxLen) / 2;
        return s.substring(start, start + maxLen);
    }

    public static void main(String[] args) {
        System.out.println(longestPalindrome("aabcdcb"));
    }
}
