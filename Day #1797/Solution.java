// Word wrap greedily: pack max words per line <= k, return null if any word > k. O(total length) time.
import java.util.*;

public class Solution {
    static List<String> wordWrap(String s, int k) {
        List<String> lines = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        for (String w : s.split(" ")) {
            if (w.length() > k) return null;
            if (cur.length() == 0) cur.append(w);
            else if (cur.length() + 1 + w.length() <= k) cur.append(" ").append(w);
            else { lines.add(cur.toString()); cur = new StringBuilder(w); }
        }
        if (cur.length() > 0) lines.add(cur.toString());
        return lines;
    }

    public static void main(String[] args) {
        List<String> res = wordWrap("the quick brown fox jumps over the lazy dog", 10);
        if (res == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("\"").append(res.get(i)).append("\"");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
