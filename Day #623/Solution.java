// Full text justification: greedily pack words per line, distribute spaces evenly
// with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).
import java.util.*;

public class Solution {
    static List<String> justify(String[] words, int k) {
        List<String> res = new ArrayList<>();
        int n = words.length, i = 0;
        while (i < n) {
            int j = i, lineLen = words[i].length();
            while (j + 1 < n && lineLen + 1 + words[j + 1].length() <= k) {
                j++;
                lineLen += 1 + words[j].length();
            }
            int gaps = j - i;
            StringBuilder line = new StringBuilder();
            if (gaps == 0) {
                line.append(words[i]);
                while (line.length() < k) line.append(' ');
            } else {
                int totalSpaces = k;
                for (int w = i; w <= j; w++) totalSpaces -= words[w].length();
                int base = totalSpaces / gaps, extra = totalSpaces % gaps;
                for (int w = i; w <= j; w++) {
                    line.append(words[w]);
                    if (w < j) {
                        int sp = base + (w - i < extra ? 1 : 0);
                        for (int s = 0; s < sp; s++) line.append(' ');
                    }
                }
            }
            res.add(line.toString());
            i = j + 1;
        }
        return res;
    }

    public static void main(String[] args) {
        String[] words = {"the","quick","brown","fox","jumps","over","the","lazy","dog"};
        for (String line : justify(words, 16))
            System.out.println('"' + line + '"');
    }
}
