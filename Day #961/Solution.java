// Day 961: Normalize an absolute Unix path resolving "." and "..".
// Approach: split by '/', use a deque/stack. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static String simplifyPath(String path) {
        Deque<String> st = new ArrayDeque<>();
        for (String seg : path.split("/")) {
            if (seg.isEmpty() || seg.equals(".")) continue;
            if (seg.equals("..")) { if (!st.isEmpty()) st.pollLast(); }
            else st.addLast(seg);
        }
        StringBuilder sb = new StringBuilder("/");
        Iterator<String> it = st.iterator();
        while (it.hasNext()) {
            sb.append(it.next());
            if (it.hasNext()) sb.append("/");
        }
        String res = sb.toString();
        if (!path.isEmpty() && path.charAt(path.length() - 1) == '/' && !res.equals("/"))
            res += "/";
        return res;
    }

    public static void main(String[] args) {
        System.out.println("\"" + simplifyPath("/usr/bin/../bin/./scripts/../") + "\"");
    }
}
