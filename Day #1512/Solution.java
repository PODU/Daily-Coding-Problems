// First recurring character: scan L->R, track seen set; first char already seen wins.
// O(n) time, O(alphabet) space.
import java.util.HashSet;
import java.util.Set;

public class Solution {
    // Returns the first recurring char as Character, or null if none.
    static Character firstRecurring(String s) {
        Set<Character> seen = new HashSet<>();
        for (char c : s.toCharArray()) {
            if (seen.contains(c)) return c;
            seen.add(c);
        }
        return null;
    }

    public static void main(String[] args) {
        Character r = firstRecurring("acbbac");
        System.out.println(r == null ? "null" : r.toString());
    }
}
