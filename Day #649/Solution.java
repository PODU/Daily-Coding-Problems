// First recurring character: single pass with a hash set, return first char already seen.
// Time O(n), Space O(k).
import java.util.HashSet;
import java.util.Set;

public class Solution {
    // Returns the recurring char, or null if none.
    static Character firstRecurring(String s) {
        Set<Character> seen = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (seen.contains(c)) return c;
            seen.add(c);
        }
        return null;
    }

    static void run(String s) {
        Character r = firstRecurring(s);
        System.out.println(r == null ? "null" : r.toString());
    }

    public static void main(String[] args) {
        run("acbbac");
        run("abcdef");
    }
}
