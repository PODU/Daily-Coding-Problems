// Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.
import java.util.*;

public class Solution {
    static List<String> ladder(String start, String end, Set<String> dict) {
        if (start.equals(end)) return Arrays.asList(start);
        Queue<String> q = new LinkedList<>();
        q.add(start);
        Map<String, String> parent = new HashMap<>();
        parent.put(start, "");
        while (!q.isEmpty()) {
            String cur = q.poll();
            char[] arr = cur.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                char orig = arr[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    if (c == orig) continue;
                    arr[i] = c;
                    String nxt = new String(arr);
                    if (dict.contains(nxt) && !parent.containsKey(nxt)) {
                        parent.put(nxt, cur);
                        if (nxt.equals(end)) {
                            LinkedList<String> path = new LinkedList<>();
                            for (String s = end; !s.isEmpty(); s = parent.get(s)) path.addFirst(s);
                            return path;
                        }
                        q.add(nxt);
                    }
                }
                arr[i] = orig;
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
        System.out.println(format(ladder("dog", "cat",
            new HashSet<>(Arrays.asList("dot", "dop", "dat", "cat")))));
        System.out.println(format(ladder("dog", "cat",
            new HashSet<>(Arrays.asList("dot", "tod", "dat", "dar")))));
    }
}
