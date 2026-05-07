// Day 1481: Reverse words while keeping delimiters in their original positions.
// Tokenize into word-runs and delimiter chars, reverse word tokens, re-emit.
// Handles trailing/consecutive delimiters. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static String reverseWords(String s, Set<Character> delims) {
        List<String> texts = new ArrayList<>();
        List<Boolean> isWord = new ArrayList<>();
        int i = 0, n = s.length();
        while (i < n) {
            if (delims.contains(s.charAt(i))) {
                texts.add(String.valueOf(s.charAt(i)));
                isWord.add(false);
                ++i;
            } else {
                int j = i;
                while (j < n && !delims.contains(s.charAt(j))) ++j;
                texts.add(s.substring(i, j));
                isWord.add(true);
                i = j;
            }
        }
        List<String> words = new ArrayList<>();
        for (int k = 0; k < texts.size(); ++k) if (isWord.get(k)) words.add(texts.get(k));
        Collections.reverse(words);
        StringBuilder out = new StringBuilder();
        int w = 0;
        for (int k = 0; k < texts.size(); ++k)
            out.append(isWord.get(k) ? words.get(w++) : texts.get(k));
        return out.toString();
    }

    public static void main(String[] args) {
        Set<Character> d = new HashSet<>(Arrays.asList('/', ':'));
        System.out.println(reverseWords("hello/world:here", d));   // here/world:hello
        System.out.println(reverseWords("hello/world:here/", d));  // here/world:hello/
        System.out.println(reverseWords("hello//world:here", d));  // here//world:hello
    }
}
