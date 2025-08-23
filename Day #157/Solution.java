// Day 157: A permutation is a palindrome iff at most one char has odd count.
// Track parity via a hash set of odd-count chars. Time O(n), Space O(alphabet).
import java.util.*;

public class Solution {
    static boolean canFormPalindrome(String s) {
        Set<Character> odd = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (!odd.add(c)) odd.remove(c);
        }
        return odd.size() <= 1;
    }

    public static void main(String[] args) {
        System.out.println(canFormPalindrome("carrace")); // true
        System.out.println(canFormPalindrome("daily"));    // false
    }
}
