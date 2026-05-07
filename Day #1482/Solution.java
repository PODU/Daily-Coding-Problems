// Day 1482: Integer palindrome without converting to a string.
// Reverse the number arithmetically and compare. Time O(digits), Space O(1).
public class Solution {
    static boolean isPalindrome(long x) {
        if (x < 0) return false;
        long rev = 0, n = x;
        while (n > 0) {
            rev = rev * 10 + n % 10;
            n /= 10;
        }
        return rev == x;
    }

    public static void main(String[] args) {
        for (long v : new long[]{121, 888, 678})
            System.out.println(v + " -> " + (isPalindrome(v) ? "palindrome" : "not a palindrome"));
    }
}
