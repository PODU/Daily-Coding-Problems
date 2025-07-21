// Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
// Time: O(n), Space: O(1).
public class Solution {
    static int numDecodings(String s) {
        if (s.isEmpty() || s.charAt(0) == '0') return 0;
        int prev2 = 1, prev1 = 1;
        for (int i = 1; i < s.length(); i++) {
            int cur = 0;
            if (s.charAt(i) != '0') cur += prev1;
            int two = (s.charAt(i - 1) - '0') * 10 + (s.charAt(i) - '0');
            if (two >= 10 && two <= 26) cur += prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        return prev1;
    }

    public static void main(String[] args) {
        System.out.println(numDecodings("111"));
    }
}
