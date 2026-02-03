// Word ladder: BFS over words, change one letter per step, track predecessors.
// Time: O(N * L * 26), Space: O(N). Returns shortest path or null.
import java.util.*;

public class Solution {
    static List<String> ladder(String start, String end, Set<String> dict) {
        if (start.equals(end)) return new ArrayList<>(List.of(start));
        Queue<String> q = new LinkedList<>();
        q.add(start);
        Map<String, String> parent = new HashMap<>();
        parent.put(start, null);
        Set<String> visited = new HashSet<>();
        visited.add(start);

        while (!q.isEmpty()) {
            String cur = q.poll();
            char[] arr = cur.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                char orig = arr[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    if (c == orig) continue;
                    arr[i] = c;
                    String nxt = new String(arr);
                    if (dict.contains(nxt) && !visited.contains(nxt)) {
                        visited.add(nxt);
                        parent.put(nxt, cur);
                        if (nxt.equals(end)) {
                            LinkedList<String> path = new LinkedList<>();
                            for (String w = end; w != null; w = parent.get(w)) path.addFirst(w);
                            return path;
                        }
                        q.add(nxt);
                    }
                }
                arr[i] = orig;
            }
        }
        return null;  // no path
    }

    static void printPath(List<String> p) {
        if (p == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < p.size(); i++) {
            sb.append("\"").append(p.get(i)).append("\"");
            if (i + 1 < p.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        printPath(ladder("dog", "cat", new HashSet<>(Arrays.asList("dot", "dop", "dat", "cat")))); // ["dog", "dot", "dat", "cat"]
        printPath(ladder("dog", "cat", new HashSet<>(Arrays.asList("dot", "tod", "dat", "dar")))); // null
    }
}
