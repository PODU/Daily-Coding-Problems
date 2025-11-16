// Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
// Time: O(N * L * 26). Space: O(N * L).
import java.util.*;

public class Solution {
    public static List<String> wordLadder(String start, String end, Set<String> dict) {
        if (!dict.contains(end)) return null;
        Queue<String> q = new LinkedList<>();
        Map<String, String> parent = new HashMap<>();
        q.add(start);
        parent.put(start, null);
        while (!q.isEmpty()) {
            String cur = q.poll();
            if (cur.equals(end)) {
                LinkedList<String> path = new LinkedList<>();
                for (String w = end; w != null; w = parent.get(w)) path.addFirst(w);
                return path;
            }
            char[] arr = cur.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                char orig = arr[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    if (c == orig) continue;
                    arr[i] = c;
                    String nxt = new String(arr);
                    if (dict.contains(nxt) && !parent.containsKey(nxt)) {
                        parent.put(nxt, cur);
                        q.add(nxt);
                    }
                }
                arr[i] = orig;
            }
        }
        return null;
    }

    static String fmt(List<String> path) {
        if (path == null) return "null";
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < path.size(); i++) {
            sb.append("\"").append(path.get(i)).append("\"");
            if (i + 1 < path.size()) sb.append(", ");
        }
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        System.out.println(fmt(wordLadder("dog", "cat",
                new HashSet<>(Arrays.asList("dot", "dop", "dat", "cat")))));
        System.out.println(fmt(wordLadder("dog", "cat",
                new HashSet<>(Arrays.asList("dot", "tod", "dat", "dar")))));
    }
}
