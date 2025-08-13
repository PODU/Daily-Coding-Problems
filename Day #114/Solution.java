// Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
// re-emit walking original structure. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static String reverseKeepDelims(String s, Set<Character> delim){
        List<String> words = new ArrayList<>();
        StringBuilder cur = new StringBuilder();
        for (char c : s.toCharArray()){
            if (delim.contains(c)){
                if (cur.length() > 0){ words.add(cur.toString()); cur.setLength(0); }
            } else cur.append(c);
        }
        if (cur.length() > 0) words.add(cur.toString());
        Collections.reverse(words);
        StringBuilder out = new StringBuilder();
        int wi = 0, i = 0, n = s.length();
        while (i < n){
            if (delim.contains(s.charAt(i))){ out.append(s.charAt(i)); i++; }
            else {
                out.append(words.get(wi++));
                while (i < n && !delim.contains(s.charAt(i))) i++;
            }
        }
        return out.toString();
    }
    public static void main(String[] args){
        Set<Character> d = new HashSet<>(Arrays.asList('/', ':'));
        System.out.println(reverseKeepDelims("hello/world:here", d));  // here/world:hello
        System.out.println(reverseKeepDelims("hello/world:here/", d)); // here/world:hello/
        System.out.println(reverseKeepDelims("hello//world:here", d)); // here//world:hello
    }
}
