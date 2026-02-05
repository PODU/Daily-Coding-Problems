// Day 1026: Full text justification.
// Greedy line packing; distribute extra spaces evenly, leftover from the left.
// Time O(total chars), Space O(total chars).
import java.util.*;

public class Solution {
    static List<String> justify(String[] words, int k) {
        List<String> res = new ArrayList<>();
        int n = words.length, i = 0;
        while (i < n) {
            int j = i, lineLen = words[i].length();
            while (j + 1 < n && lineLen + 1 + words[j + 1].length() <= k) {
                lineLen += 1 + words[j + 1].length();
                j++;
            }
            int cnt = j - i + 1;
            StringBuilder line = new StringBuilder();
            if (cnt == 1) {
                line.append(words[i]);
                while (line.length() < k) line.append(' ');
            } else {
                int totalChars = 0;
                for (int w = i; w <= j; w++) totalChars += words[w].length();
                int spaces = k - totalChars, gaps = cnt - 1;
                int base = spaces / gaps, extra = spaces % gaps;
                for (int w = i; w <= j; w++) {
                    line.append(words[w]);
                    if (w < j) {
                        int s = base + (w - i < extra ? 1 : 0);
                        for (int t = 0; t < s; t++) line.append(' ');
                    }
                }
            }
            res.add(line.toString());
            i = j + 1;
        }
        return res;
    }

    public static void main(String[] args) {
        String[] words = {"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"};
        for (String line : justify(words, 16)) System.out.println(line);
    }
}
