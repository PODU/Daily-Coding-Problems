// Reverse words while keeping delimiters in place: split into word/delimiter tokens,
// reverse only the word list, re-emit in original token order. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static String reverseWords(String s, Set<Character> delims) {
        List<String> tokens = new ArrayList<>();
        List<Boolean> isWord = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        for (char c : s.toCharArray()) {
            if (delims.contains(c)) {
                if (cur.length() > 0) { tokens.add(cur.toString()); isWord.add(true); cur.setLength(0); }
                tokens.add(String.valueOf(c)); isWord.add(false);
            } else {
                cur.append(c);
            }
        }
        if (cur.length() > 0) { tokens.add(cur.toString()); isWord.add(true); }

        List<String> words = new ArrayList<>();
        for (int i = 0; i < tokens.size(); i++)
            if (isWord.get(i)) words.add(tokens.get(i));
        Collections.reverse(words);

        StringBuilder res = new StringBuilder();
        int wi = 0;
        for (int i = 0; i < tokens.size(); i++)
            res.append(isWord.get(i) ? words.get(wi++) : tokens.get(i));
        return res.toString();
    }

    public static void main(String[] args) {
        Set<Character> delims = new HashSet<>(Arrays.asList('/', ':'));
        System.out.println(reverseWords("hello/world:here", delims));
        System.out.println(reverseWords("hello/world:here/", delims));
        System.out.println(reverseWords("hello//world:here", delims));
    }
}
