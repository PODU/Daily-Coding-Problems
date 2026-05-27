// Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.
import java.util.*;

public class Solution {
    static List<String> wordBreak(String s, Set<String> dict) {
        int n = s.length();
        int[] back = new int[n + 1];
        Arrays.fill(back, -2); // -2 unreachable
        back[0] = -1;
        for (int i = 1; i <= n; i++)
            for (int j = 0; j < i; j++)
                if (back[j] != -2 && dict.contains(s.substring(j, i))) { back[i] = j; break; }
        if (back[n] == -2) return null;
        LinkedList<String> res = new LinkedList<>();
        for (int i = n; i > 0; i = back[i]) res.addFirst(s.substring(back[i], i));
        return res;
    }

    public static void main(String[] args) {
        Set<String> dict = new HashSet<>(Arrays.asList("quick", "brown", "the", "fox"));
        List<String> res = wordBreak("thequickbrownfox", dict);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++)
            sb.append("'").append(res.get(i)).append("'").append(i + 1 < res.size() ? ", " : "");
        sb.append("]");
        System.out.println(sb.toString());
    }
}
