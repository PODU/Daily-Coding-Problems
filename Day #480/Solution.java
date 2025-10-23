// Word break reconstruction via memoized DP: for each suffix, try each prefix
// word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Solution {
    static List<String> solve(String s, int start, Set<String> dict,
                              Map<Integer, List<String>> memo) {
        if (start == s.length()) return new ArrayList<>();
        if (memo.containsKey(start)) return memo.get(start);
        for (int end = start + 1; end <= s.length(); end++) {
            String word = s.substring(start, end);
            if (dict.contains(word)) {
                List<String> rest = solve(s, end, dict, memo);
                if (rest != null) {
                    List<String> res = new ArrayList<>();
                    res.add(word);
                    res.addAll(rest);
                    memo.put(start, res);
                    return res;
                }
            }
        }
        memo.put(start, null);
        return null;
    }

    static void run(Set<String> dict, String s) {
        List<String> res = solve(s, 0, dict, new HashMap<>());
        if (res == null) {
            System.out.println("null");
            return;
        }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append('\'').append(res.get(i)).append('\'');
        }
        sb.append(']');
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        run(new HashSet<>(Arrays.asList("quick", "brown", "the", "fox")), "thequickbrownfox");
        run(new HashSet<>(Arrays.asList("bed", "bath", "bedbath", "and", "beyond")), "bedbathandbeyond");
    }
}
