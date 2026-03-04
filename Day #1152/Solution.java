// Day 1152: Simplify absolute Unix path.
// Stack of components; '.' ignored, '..' pops. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static String simplify(String path) {
        Deque<String> st = new ArrayDeque<>();
        for (String part : path.split("/")) {
            if (part.isEmpty() || part.equals(".")) continue;
            if (part.equals("..")) { if (!st.isEmpty()) st.removeLast(); }
            else st.addLast(part);
        }
        StringBuilder sb = new StringBuilder("/");
        for (String p : st) sb.append(p).append("/");
        return sb.toString(); // trailing slash preserved (matches example)
    }

    public static void main(String[] args) {
        System.out.println(simplify("/usr/bin/../bin/./scripts/../")); // /usr/bin/
    }
}
