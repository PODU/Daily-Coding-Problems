// Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
// Approach: greedy single pass over words. O(n) time, O(n) space. null if a word > k.
import java.util.*;

public class Solution {
    static List<String> wrap(String s, int k) {
        List<String> out = new ArrayList<>();
        StringBuilder line = new StringBuilder();
        for (String word : s.split(" ")) {
            if (word.length() > k) return null; // impossible
            if (line.length() == 0) line.append(word);
            else if (line.length() + 1 + word.length() <= k) line.append(" ").append(word);
            else { out.add(line.toString()); line = new StringBuilder(word); }
        }
        if (line.length() > 0) out.add(line.toString());
        return out;
    }

    public static void main(String[] args) {
        List<String> out = wrap("the quick brown fox jumps over the lazy dog", 10);
        if (out == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < out.size(); i++) {
            sb.append("\"").append(out.get(i)).append("\"");
            if (i + 1 < out.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
