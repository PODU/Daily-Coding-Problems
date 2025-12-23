// Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.
public class Solution {
    static boolean isPalindrome(long n) {
        if (n < 0) return false;
        long rev = 0, x = n;
        while (x > 0) {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        return rev == n;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(121));
    }
}
