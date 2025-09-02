// Day 202: Integer palindrome check without string conversion.
// Reverse the second half of the digits and compare with the first half.
// Time: O(log10 n), Space: O(1).
public class Solution {
    static boolean isPalindrome(long x) {
        if (x < 0) return false;
        if (x != 0 && x % 10 == 0) return false; // trailing zero, not 0 itself
        long rev = 0;
        while (x > rev) { rev = rev * 10 + x % 10; x /= 10; }
        return x == rev || x == rev / 10;
    }

    public static void main(String[] args) {
        System.out.println(isPalindrome(121)); // true
        System.out.println(isPalindrome(888)); // true
        System.out.println(isPalindrome(678)); // false
    }
}
