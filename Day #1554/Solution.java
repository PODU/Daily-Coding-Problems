// Palindrome permutation: toggle chars in a set; a permutation is a palindrome
// iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).
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
        String s = "carrace";
        System.out.println(canFormPalindrome(s) ? "true" : "false");
    }
}
