// Day 837: Sentence checker over a character stream.
// Accumulate chars until a terminal mark, then validate the buffered sentence by regex; print if valid.
// O(N) over the stream; O(L) per sentence buffer.
import java.util.*;
import java.util.regex.*;

public class Solution {
    static final String TERMINALS = ".?!‽"; // . ? ! ‽
    static final Pattern PATTERN =
            Pattern.compile("^[A-Z][a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$");

    static List<String> checkStream(String stream) {
        List<String> results = new ArrayList<>();
        StringBuilder buf = new StringBuilder();
        for (int i = 0; i < stream.length(); i++) {
            char ch = stream.charAt(i);
            buf.append(ch);
            if (TERMINALS.indexOf(ch) >= 0) {
                String sentence = buf.toString();
                int p = 0;
                while (p < sentence.length() && sentence.charAt(p) == ' ') p++;
                sentence = sentence.substring(p);
                if (PATTERN.matcher(sentence).matches()) results.add(sentence);
                buf.setLength(0);
            }
        }
        return results;
    }

    public static void main(String[] args) {
        String stream = "Hello, world. this is wrong. The cat sat.";
        for (String s : checkStream(stream)) System.out.println(s);
    }
}
