// Day 713: Normalize absolute Unix path. Split on '/', use a stack: skip ""/".",
// pop on "..". Preserve a trailing slash if the input had one. Time O(n).
import java.util.*;

public class Solution {
    static String normalize(String path) {
        Deque<String> stk = new ArrayDeque<>();
        for (String comp : path.split("/")) {
            if (comp.isEmpty() || comp.equals(".")) continue;
            if (comp.equals("..")) { if (!stk.isEmpty()) stk.removeLast(); }
            else stk.addLast(comp);
        }
        StringBuilder sb = new StringBuilder("/");
        sb.append(String.join("/", stk));
        String res = sb.toString();
        boolean trailing = path.length() > 1 && path.charAt(path.length() - 1) == '/';
        if (trailing && !res.equals("/") && res.charAt(res.length() - 1) != '/') res += "/";
        return res;
    }

    public static void main(String[] args) {
        System.out.println("\"" + normalize("/usr/bin/../bin/./scripts/../") + "\"");
    }
}
