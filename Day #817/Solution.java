// Word break via DP with backpointers: dp[i] reachable, prev[i] start of last word. O(n^2) time, O(n) space.
import java.util.*;

public class Solution {
    static List<String> wordBreak(String s, Set<String> dict) {
        int n = s.length();
        int[] prev = new int[n + 1];
        Arrays.fill(prev, -2); // -2 = unreachable
        prev[0] = -1;
        for (int i = 1; i <= n; i++) {
            for (int j = i - 1; j >= 0; j--) { // prefer shortest last word
                if (prev[j] != -2 && dict.contains(s.substring(j, i))) {
                    prev[i] = j;
                    break;
                }
            }
        }
        if (prev[n] == -2) return null;
        LinkedList<String> res = new LinkedList<>();
        for (int i = n; i > 0; i = prev[i])
            res.addFirst(s.substring(prev[i], i));
        return res;
    }

    static String fmt(List<String> r) {
        return r == null ? "null" : "[" + String.join(", ", r) + "]";
    }

    public static void main(String[] args) {
        System.out.println(fmt(wordBreak("thequickbrownfox",
                new HashSet<>(Arrays.asList("quick", "brown", "the", "fox")))));
        System.out.println(fmt(wordBreak("bedbathandbeyond",
                new HashSet<>(Arrays.asList("bed", "bath", "bedbath", "and", "beyond")))));
    }
}
