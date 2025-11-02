// Permutation-palindrome check: a permutation can be a palindrome iff at most one
// character has an odd frequency. Toggle membership in a set as we scan.
// Time: O(n); Space: O(alphabet).
import java.util.*;

public class Solution {
    static boolean canPermutePalindrome(String s) {
        Set<Character> odd = new HashSet<>();
        for (char ch : s.toCharArray()) {
            if (!odd.add(ch)) odd.remove(ch);
        }
        return odd.size() <= 1;
    }

    public static void main(String[] args) {
        System.out.println(canPermutePalindrome("carrace"));
        System.out.println(canPermutePalindrome("daily"));
    }
}
