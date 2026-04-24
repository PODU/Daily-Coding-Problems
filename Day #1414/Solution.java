// Day 1414: does a one-to-one (bijective) char mapping s1 -> s2 exist?
// Approach: enforce a consistent forward map AND injective (no two srcs share a target). O(n).
import java.util.*;

public class Solution {
    static boolean canMap(String s1, String s2) {
        if (s1.length() != s2.length()) return false;
        Map<Character,Character> fwd = new HashMap<>(), rev = new HashMap<>();
        for (int i = 0; i < s1.length(); i++) {
            char a = s1.charAt(i), b = s2.charAt(i);
            if (fwd.containsKey(a) && fwd.get(a) != b) return false;
            if (rev.containsKey(b) && rev.get(b) != a) return false;
            fwd.put(a, b); rev.put(b, a);
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(canMap("abc", "bcd") ? "true" : "false"); // true
        System.out.println(canMap("foo", "bar") ? "true" : "false"); // false
    }
}
