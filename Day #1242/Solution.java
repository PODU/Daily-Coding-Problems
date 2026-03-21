// Word ladder: BFS over words differing by one letter. Time O(N*L*26).
import java.util.*;

public class Solution {
    static List<String> wordLadder(String start, String end, Set<String> dict) {
        if (!dict.contains(end)) return null;
        Queue<List<String>> q = new LinkedList<>();
        q.add(new ArrayList<>(List.of(start)));
        Set<String> seen = new HashSet<>();
        seen.add(start);
        while (!q.isEmpty()) {
            List<String> path = q.poll();
            String word = path.get(path.size() - 1);
            if (word.equals(end)) return path;
            char[] arr = word.toCharArray();
            for (int i = 0; i < arr.length; i++) {
                char orig = arr[i];
                for (char c = 'a'; c <= 'z'; c++) {
                    arr[i] = c;
                    String nxt = new String(arr);
                    if (dict.contains(nxt) && !seen.contains(nxt)) {
                        seen.add(nxt);
                        List<String> np = new ArrayList<>(path);
                        np.add(nxt);
                        q.add(np);
                    }
                }
                arr[i] = orig;
            }
        }
        return null;
    }

    public static void main(String[] args) {
        System.out.println(wordLadder("dog", "cat", new HashSet<>(Set.of("dot", "dop", "dat", "cat"))));
        System.out.println(wordLadder("dog", "cat", new HashSet<>(Set.of("dot", "tod", "dat", "dar"))));
    }
}
