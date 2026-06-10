// Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class Solution {
    static boolean differsByOne(String a, String b) {
        if (a.length() != b.length()) return false;
        int diff = 0;
        for (int i = 0; i < a.length(); i++)
            if (a.charAt(i) != b.charAt(i) && ++diff > 1) return false;
        return diff == 1;
    }

    // returns null if no path
    static List<String> ladder(String start, String end, String[] dict) {
        Set<String> visited = new HashSet<>();
        visited.add(start);
        Queue<String> q = new ArrayDeque<>();
        q.add(start);
        Map<String, String> parent = new HashMap<>();
        while (!q.isEmpty()) {
            String cur = q.poll();
            if (cur.equals(end)) {
                List<String> path = new ArrayList<>();
                String at = cur;
                while (true) {
                    path.add(at);
                    if (at.equals(start)) break;
                    at = parent.get(at);
                }
                Collections.reverse(path);
                return path;
            }
            for (String w : dict) {
                if (!visited.contains(w) && differsByOne(cur, w)) {
                    visited.add(w);
                    parent.put(w, cur);
                    q.add(w);
                }
            }
        }
        return null;
    }

    static String format(List<String> path) {
        if (path == null) return "null";
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < path.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append("\"").append(path.get(i)).append("\"");
        }
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        System.out.println(format(ladder("dog", "cat", new String[]{"dot", "dop", "dat", "cat"})));
        System.out.println(format(ladder("dog", "cat", new String[]{"tod", "dat", "dar", "dot"})));
    }
}
