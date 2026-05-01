// Day 1456: First recurring character in a string.
// Approach: scan left-to-right, track seen chars in a HashSet; first char
// already seen is the answer. Time O(n), Space O(1) (fixed alphabet).
import java.util.*;

public class Solution {
    // Returns the first recurring character, or null if none.
    static Character firstRecurring(String s) {
        Set<Character> seen = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (seen.contains(c)) return c;
            seen.add(c);
        }
        return null;
    }

    public static void main(String[] args) {
        for (String s : new String[]{"acbbac", "abcdef"}) {
            Character c = firstRecurring(s);
            System.out.println(c == null ? "null" : "\"" + c + "\"");
        }
    }
}
