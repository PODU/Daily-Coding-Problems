// Day 431: Sentence validator via finite-state-machine scan.
// Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!‽]$ (no backtracking needed).
// O(n) time, O(1) extra space per sentence.
public class Solution {
    static boolean isSep(char c)  { return c == ',' || c == ';' || c == ':'; }
    static boolean isTerm(char c) { return c == '.' || c == '?' || c == '!' || c == '‽'; }
    static boolean isLow(char c)  { return c >= 'a' && c <= 'z'; }

    static boolean isValidSentence(String s) {
        int n = s.length();
        if (n == 0) return false;
        if (!(s.charAt(0) >= 'A' && s.charAt(0) <= 'Z')) return false;
        int i = 1;
        while (i < n && isLow(s.charAt(i))) i++;
        while (true) {
            int j = i;
            if (j < n && isSep(s.charAt(j))) j++;
            if (j < n && s.charAt(j) == ' ') {
                j++;
                if (j < n && isLow(s.charAt(j))) {
                    while (j < n && isLow(s.charAt(j))) j++;
                    i = j;
                    continue;
                }
            }
            break;
        }
        if (i < n && isSep(s.charAt(i))) i++;
        return i == n - 1 && isTerm(s.charAt(i));
    }

    public static void main(String[] args) {
        String[] tests = {"The quick brown fox.", "hello world.", "Hello  world.",
                          "Hello world", "Hi there, friend!"};
        for (String t : tests)
            System.out.println((isValidSentence(t) ? "Valid: " : "Invalid: ") + t);
    }
}
