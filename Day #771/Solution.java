// Day 771: One-to-one (bijective) character mapping between s1 and s2.
// Track forward and reverse maps; reject conflicts. O(n) time, O(1) space.
import java.util.*;

public class Solution {
    static boolean isOneToOne(String s1, String s2) {
        if (s1.length() != s2.length()) return false;
        Map<Character, Character> fwd = new HashMap<>(), rev = new HashMap<>();
        for (int i = 0; i < s1.length(); i++) {
            char a = s1.charAt(i), b = s2.charAt(i);
            if (fwd.containsKey(a) && fwd.get(a) != b) return false;
            if (rev.containsKey(b) && rev.get(b) != a) return false;
            fwd.put(a, b); rev.put(b, a);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isOneToOne("abc", "bcd")); // true
        System.out.println(isOneToOne("foo", "bar")); // false
    }
}
