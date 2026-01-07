// Day 868: Can any permutation of the string form a palindrome?
// Approach: count chars; palindrome possible iff at most one char has an odd count.
// Time: O(n), Space: O(alphabet).
import java.util.*;

public class Solution {
    static boolean canFormPalindrome(String s) {
        Map<Character, Integer> cnt = new HashMap<>();
        for (char c : s.toCharArray()) cnt.merge(c, 1, Integer::sum);
        int odd = 0;
        for (int v : cnt.values()) odd += v & 1;
        return odd <= 1;
    }

    public static void main(String[] args) {
        System.out.println(canFormPalindrome("carrace")); // true
        System.out.println(canFormPalindrome("daily"));   // false
    }
}
