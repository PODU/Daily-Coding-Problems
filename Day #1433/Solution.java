// Day 1433: Sentence checker over a character stream.
// Approach: finite-state machine validating one sentence at a time. Time: O(n), Space: O(n) buffer.
public class Solution {
    static boolean isTerminal(char c) {
        return c == '.' || c == '?' || c == '!' || c == '‽'; // ‽
    }

    static boolean isSeparator(char c) {
        return c == ',' || c == ';' || c == ':';
    }

    static boolean isValidSentence(String s) {
        int n = s.length();
        if (n < 2) return false;
        // Rule 1: capital then lowercase or space.
        if (!Character.isUpperCase(s.charAt(0))) return false;
        char second = s.charAt(1);
        if (!(Character.isLowerCase(second) || second == ' ')) return false;

        boolean prevWasLetter = Character.isLetter(s.charAt(0));
        for (int i = 1; i < n; i++) {
            char c = s.charAt(i);
            if (isTerminal(c)) {
                // Rule 4: terminal immediately after a word and ends the sentence.
                if (!prevWasLetter) return false;
                return i == n - 1;
            }
            if (c == ' ') {
                if (s.charAt(i - 1) == ' ') return false; // Rule 3: single space
                prevWasLetter = false;
            } else if (Character.isLowerCase(c)) {
                prevWasLetter = true;
            } else if (isSeparator(c)) {
                prevWasLetter = false;
            } else {
                return false;
            }
        }
        return false; // no terminal mark
    }

    public static void main(String[] args) {
        String[] tests = {
            "The quick brown fox.",
            "Hello world!",
            "lowercase start.",
            "No terminal mark",
            "Two  spaces here."
        };
        for (String t : tests) {
            if (isValidSentence(t)) System.out.println(t);
        }
    }
}
