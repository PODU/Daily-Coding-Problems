// Day 222: Normalize an absolute path (resolve . and ..).
// Approach: split on '/', push names onto a stack, pop on "..", skip "." and "". Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static String simplifyPath(String path) {
        Deque<String> st = new ArrayDeque<>();
        for (String tok : path.split("/")) {
            if (tok.isEmpty() || tok.equals(".")) continue;
            if (tok.equals("..")) { if (!st.isEmpty()) st.removeLast(); }
            else st.addLast(tok);
        }
        if (st.isEmpty()) return "/";
        StringBuilder sb = new StringBuilder();
        for (String s : st) sb.append("/").append(s);
        return sb.append("/").toString(); // trailing slash (directory form)
    }

    public static void main(String[] args) {
        System.out.println("\"" + simplifyPath("/usr/bin/../bin/./scripts/../") + "\""); // "/usr/bin/"
    }
}
