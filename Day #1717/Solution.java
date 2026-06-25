// Day 1717: Fully justify text into lines of length k.
// Greedy line packing + even space distribution (extras from left).
// Time: O(total characters), Space: O(output).
import java.util.*;

public class Solution {
    static List<String> justify(String[] words, int k) {
        List<String> lines = new ArrayList<>();
        int n = words.length, i = 0;
        while (i < n) {
            int j = i, lineLen = words[i].length();
            while (j + 1 < n && lineLen + 1 + words[j + 1].length() <= k) {
                j++;
                lineLen += 1 + words[j].length();
            }
            int cnt = j - i + 1, wordChars = 0;
            for (int t = i; t <= j; t++) wordChars += words[t].length();
            StringBuilder line = new StringBuilder();
            if (cnt == 1) {
                line.append(words[i]);
                while (line.length() < k) line.append(' ');
            } else {
                int gaps = cnt - 1;
                int totalSpaces = k - wordChars;
                int base = totalSpaces / gaps, extra = totalSpaces % gaps;
                for (int t = i; t <= j; t++) {
                    line.append(words[t]);
                    if (t < j) {
                        int sp = base + (t - i < extra ? 1 : 0);
                        for (int s = 0; s < sp; s++) line.append(' ');
                    }
                }
            }
            lines.add(line.toString());
            i = j + 1;
        }
        return lines;
    }

    public static void main(String[] args) {
        String[] words = {"the","quick","brown","fox","jumps","over","the","lazy","dog"};
        int k = 16;
        for (String l : justify(words, k))
            System.out.println("\"" + l + "\"");
    }
}
