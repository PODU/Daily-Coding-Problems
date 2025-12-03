// Reverse words between delimiters while keeping delimiters fixed in position.
// Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.
import java.util.*;

public class Solution {
    static String reverseWords(String s, Set<Character> delims) {
        List<String> vals = new ArrayList<>();
        List<Boolean> isDelim = new ArrayList<>();
        StringBuilder buf = new StringBuilder();
        for (char c : s.toCharArray()) {
            if (delims.contains(c)) {
                vals.add(buf.toString()); isDelim.add(false);
                vals.add(String.valueOf(c)); isDelim.add(true);
                buf.setLength(0);
            } else {
                buf.append(c);
            }
        }
        if (buf.length() > 0) { vals.add(buf.toString()); isDelim.add(false); }

        List<String> words = new ArrayList<>();
        for (int i = 0; i < vals.size(); i++) if (!isDelim.get(i)) words.add(vals.get(i));
        Collections.reverse(words);

        StringBuilder out = new StringBuilder();
        int wi = 0;
        for (int i = 0; i < vals.size(); i++) {
            if (isDelim.get(i)) out.append(vals.get(i));
            else out.append(words.get(wi++));
        }
        return out.toString();
    }

    public static void main(String[] args) {
        Set<Character> d = new HashSet<>(Arrays.asList('/', ':'));
        String[] tests = {"hello/world:here", "hello/world:here/", "hello//world:here"};
        for (String t : tests)
            System.out.println(t + " -> " + reverseWords(t, d));
    }
}
