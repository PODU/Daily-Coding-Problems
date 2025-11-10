// Bijective char mapping: maintain s1->s2 (consistent) and s2->s1 (injective) maps. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static boolean bijective(String s1, String s2) {
        if (s1.length() != s2.length()) return false;
        Map<Character, Character> fwd = new HashMap<>(), bwd = new HashMap<>();
        for (int i = 0; i < s1.length(); i++) {
            char a = s1.charAt(i), b = s2.charAt(i);
            if (fwd.containsKey(a) && fwd.get(a) != b) return false;
            if (bwd.containsKey(b) && bwd.get(b) != a) return false;
            fwd.put(a, b);
            bwd.put(b, a);
        }
        return true;
    }

    public static void main(String[] args) {
        if (bijective("abc", "bcd"))
            System.out.println("true (map a to b, b to c, and c to d)");
        if (!bijective("foo", "bar"))
            System.out.println("false (the o cannot map to two characters)");
    }
}
