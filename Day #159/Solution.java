// Day 159: First recurring character. Scan left to right tracking seen chars in
// a set; return the first already seen. Time O(n), Space O(alphabet).
import java.util.*;

public class Solution {
    // returns boxed Character, or null if none
    static Character firstRecurring(String s) {
        Set<Character> seen = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (!seen.add(c)) return c;
        }
        return null;
    }

    public static void main(String[] args) {
        Character r1 = firstRecurring("acbbac");
        Character r2 = firstRecurring("abcdef");
        System.out.println(r1 == null ? "null" : r1); // b
        System.out.println(r2 == null ? "null" : r2); // null
    }
}
