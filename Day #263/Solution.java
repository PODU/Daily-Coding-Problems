// Day 263: Sentence checker over a stream of characters.
// Approach: scan the stream into candidate sentences (cut at terminal marks),
// validate each with a regex enforcing all four rules. Time O(n) per sentence.

import java.util.*;
import java.util.regex.*;

public class Solution {
    // Rules:
    // 1. Starts with a capital letter followed by a lowercase letter or a space.
    // 2. Other chars: lowercase letters, separators (, ; :) or terminal marks (. ? ! ‽).
    // 3. A single space between each word.
    // 4. Ends with a terminal mark immediately following a word.
    private static final Pattern RE =
        Pattern.compile("^[A-Z](?=[a-z ])[a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!‽]$");

    static boolean isValid(String s) {
        return RE.matcher(s).matches();
    }

    public static void main(String[] args) {
        String[] tests = {
            "Hello world.",
            "hello world",
            "Hello  world.",
            "The quick, brown fox jumps.",
            "Is this valid?",
            "No terminal mark",
            "Bad ,spacing."
        };
        for (String t : tests) {
            if (isValid(t)) System.out.println(t);
        }
    }
}
