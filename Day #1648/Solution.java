// Simplify Unix absolute path: split on '/', push names on a stack, skip ''/'.', pop on '..'. Time O(n), Space O(n).
// Build "/a/b" from the stack; if input ended with '/' and result isn't root, append a trailing '/'.
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static String simplifyPath(String path) {
        Deque<String> stack = new ArrayDeque<>();
        String[] tokens = path.split("/", -1);
        for (String t : tokens) {
            if (t.isEmpty() || t.equals(".")) continue;
            if (t.equals("..")) {
                if (!stack.isEmpty()) stack.pollLast();
            } else {
                stack.addLast(t);
            }
        }
        StringBuilder sb = new StringBuilder("/");
        boolean first = true;
        for (String s : stack) {
            if (!first) sb.append("/");
            sb.append(s);
            first = false;
        }
        String result = sb.toString();
        boolean endsWithSlash = !path.isEmpty() && path.charAt(path.length() - 1) == '/';
        if (endsWithSlash && !result.equals("/")) result += "/";
        return result;
    }

    public static void main(String[] args) {
        System.out.println(simplifyPath("/usr/bin/../bin/./scripts/../"));
    }
}
