// Group anagrams: hash map keyed by sorted chars -> list, preserving first-seen group order.
// Time O(N*K log K), Space O(N*K).
import java.util.*;

public class Solution {
    static List<List<String>> groupAnagrams(String[] words) {
        Map<String, Integer> idx = new HashMap<>();
        List<List<String>> groups = new ArrayList<>();
        for (String w : words) {
            char[] c = w.toCharArray();
            Arrays.sort(c);
            String key = new String(c);
            Integer g = idx.get(key);
            if (g == null) {
                idx.put(key, groups.size());
                List<String> list = new ArrayList<>();
                list.add(w);
                groups.add(list);
            } else {
                groups.get(g).add(w);
            }
        }
        return groups;
    }

    public static void main(String[] args) {
        String[] input = {"eat", "ate", "apt", "pat", "tea", "now"};
        List<List<String>> groups = groupAnagrams(input);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < groups.size(); i++) {
            sb.append("[");
            List<String> g = groups.get(i);
            for (int j = 0; j < g.size(); j++) {
                sb.append("'").append(g.get(j)).append("'");
                if (j + 1 < g.size()) sb.append(", ");
            }
            sb.append("]");
            if (i + 1 < groups.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
