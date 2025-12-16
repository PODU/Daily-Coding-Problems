// Longest palindromic substring via Manacher's algorithm.
// Transform with '#' separators, expand radii using mirror symmetry.
// Time: O(n), Space: O(n).
public class Solution {
    static String longestPalindrome(String s){
        if(s.isEmpty()) return "";
        StringBuilder tb = new StringBuilder("#");
        for(char c : s.toCharArray()){ tb.append(c).append('#'); }
        String t = tb.toString();
        int n = t.length();
        int[] p = new int[n];
        int c = 0, r = 0;
        for(int i=0;i<n;i++){
            if(i < r) p[i] = Math.min(r - i, p[2*c - i]);
            while(i - p[i] - 1 >= 0 && i + p[i] + 1 < n && t.charAt(i-p[i]-1) == t.charAt(i+p[i]+1)) p[i]++;
            if(i + p[i] > r){ c = i; r = i + p[i]; }
        }
        int maxLen = 0, center = 0;
        for(int i=0;i<n;i++) if(p[i] > maxLen){ maxLen = p[i]; center = i; }
        int start = (center - maxLen) / 2;
        return s.substring(start, start + maxLen);
    }

    public static void main(String[] args){
        System.out.println(longestPalindrome("aabcdcb")); // bcdcb
        System.out.println(longestPalindrome("bananas")); // anana
    }
}
