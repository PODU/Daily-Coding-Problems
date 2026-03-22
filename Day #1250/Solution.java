// Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
import java.util.*;

public class Solution {
    static boolean canPermutePalindrome(String s) {
        Map<Character,Integer> cnt = new HashMap<>();
        for (char c : s.toCharArray()) cnt.merge(c, 1, Integer::sum);
        int odd = 0;
        for (int v : cnt.values()) if ((v & 1) == 1) odd++;
        return odd <= 1;
    }

    public static void main(String[] args) {
        System.out.println(canPermutePalindrome("carrace"));
        System.out.println(canPermutePalindrome("daily"));
    }
}
