// Day 1335: Count decodings of a digit string with a=1..z=26.
// DP: ways[i] += ways[i-1] if digit valid, += ways[i-2] if two-digit 10..26 valid. O(n) time, O(1) space.
public class Solution {
    static long numDecodings(String s) {
        int n = s.length();
        if (n == 0) return 0;
        long prev2 = 1, prev1 = (s.charAt(0) != '0') ? 1 : 0;
        for (int i = 1; i < n; i++) {
            long cur = 0;
            if (s.charAt(i) != '0') cur += prev1;
            int two = (s.charAt(i - 1) - '0') * 10 + (s.charAt(i) - '0');
            if (two >= 10 && two <= 26) cur += prev2;
            prev2 = prev1; prev1 = cur;
        }
        return prev1;
    }

    public static void main(String[] args) {
        System.out.println(numDecodings("111")); // 3
    }
}
