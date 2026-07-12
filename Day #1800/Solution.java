// Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).
public class Solution {
    static boolean isPalindrome(long x) {
        if (x < 0) return false;
        long original = x, reversed = 0;
        while (x > 0) {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }
        return reversed == original;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(121)); // true
    }
}
