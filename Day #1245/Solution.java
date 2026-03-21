// Full text justification: greedy packing + even space distribution (extras
// to the left). Time O(total chars), Space O(output).
import java.util.*;

public class Solution {
    static List<String> justify(String[] words, int k) {
        List<String> lines = new ArrayList<>();
        int i = 0, n = words.length;
        while (i < n) {
            int j = i, length = 0;
            while (j < n && length + words[j].length() + (j - i) <= k) {
                length += words[j].length();
                j++;
            }
            int gaps = j - i - 1;
            StringBuilder line = new StringBuilder();
            if (gaps == 0) {
                line.append(words[i]);
                while (line.length() < k) line.append(' ');
            } else {
                int spaces = k - length, base = spaces / gaps, extra = spaces % gaps;
                for (int w = i; w < j - 1; w++) {
                    line.append(words[w]);
                    int sp = base + ((w - i) < extra ? 1 : 0);
                    for (int s = 0; s < sp; s++) line.append(' ');
                }
                line.append(words[j - 1]);
            }
            lines.add(line.toString());
            i = j;
        }
        return lines;
    }

    public static void main(String[] args) {
        String[] words = {"the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"};
        for (String line : justify(words, 16)) System.out.println("\"" + line + "\"");
    }
}
