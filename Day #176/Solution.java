// Bijective char mapping check: track s1->s2 map and set of used s2 chars; reject conflicts.
// Time O(n), Space O(unique chars).
import java.util.*;

public class Solution {
    public static boolean isBijective(String s1, String s2) {
        if (s1.length() != s2.length()) return false;
        Map<Character, Character> mapping = new HashMap<>();
        Set<Character> used = new HashSet<>();
        for (int i = 0; i < s1.length(); i++) {
            char a = s1.charAt(i), b = s2.charAt(i);
            if (mapping.containsKey(a)) {
                if (mapping.get(a) != b) return false;
            } else {
                if (used.contains(b)) return false;
                mapping.put(a, b);
                used.add(b);
            }
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isBijective("abc", "bcd"));
        System.out.println(isBijective("foo", "bar"));
    }
}
