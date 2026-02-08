// Reverse words but keep delimiters fixed in place: extract words, reverse the list,
// rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static Set<Character> DELIMS = new HashSet<>(Arrays.asList('/', ':'));

    static boolean isDelim(char c) { return DELIMS.contains(c); }

    static String reverseWords(String s) {
        List<String> words = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        for (char c : s.toCharArray()) {
            if (isDelim(c)) {
                if (cur.length() > 0) { words.add(cur.toString()); cur.setLength(0); }
            } else {
                cur.append(c);
            }
        }
        if (cur.length() > 0) words.add(cur.toString());
        Collections.reverse(words);

        StringBuilder out = new StringBuilder();
        int wi = 0, i = 0, n = s.length();
        while (i < n) {
            if (isDelim(s.charAt(i))) {
                out.append(s.charAt(i));
                i++;
            } else {
                out.append(words.get(wi++));
                while (i < n && !isDelim(s.charAt(i))) i++;
            }
        }
        return out.toString();
    }

    public static void main(String[] args) {
        String[] tests = {"hello/world:here", "hello/world:here/", "hello//world:here"};
        for (String t : tests)
            System.out.println(t + " -> " + reverseWords(t));
    }
}
