// Day 934: First recurring character = first char that has been seen before while scanning.
// Hash set of seen chars; return on first repeat. Time O(n), Space O(min(n, alphabet)).
import java.util.HashSet;
import java.util.Set;

public class Solution {
    // Returns the recurring char as a Character, or null if none.
    static Character firstRecurring(String s) {
        Set<Character> seen = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (seen.contains(c)) return c;
            seen.add(c);
        }
        return null;
    }

    public static void main(String[] args) {
        String[] inputs = {"acbbac", "abcdef"};
        for (String s : inputs) {
            Character c = firstRecurring(s);
            System.out.println(c == null ? "null" : "\"" + c + "\"");
        }
        // "b"
        // null
    }
}
