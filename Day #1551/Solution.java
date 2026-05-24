// Sentence validator over a char stream: split on terminal marks, validate each candidate.
// isValid checks capital start, lowercase/separators body, single spaces, terminal end. Time O(n).
public class Solution {
    static boolean isTerminal(char c) { return c == '.' || c == '?' || c == '!' || c == '‽'; }
    static boolean isSeparator(char c) { return c == ',' || c == ';' || c == ':'; }
    static boolean isLower(char c) { return c >= 'a' && c <= 'z'; }
    static boolean isUpper(char c) { return c >= 'A' && c <= 'Z'; }

    static boolean isValid(String s) {
        int n = s.length();
        if (n < 2) return false;
        if (!isUpper(s.charAt(0))) return false;
        if (!(isLower(s.charAt(1)) || s.charAt(1) == ' ')) return false;
        if (!isTerminal(s.charAt(n - 1))) return false;
        if (!(isLower(s.charAt(n - 2)) || isUpper(s.charAt(n - 2)))) return false;
        for (int i = 1; i + 1 < n; i++) {
            char c = s.charAt(i);
            if (isLower(c) || isSeparator(c)) continue;
            if (c == ' ') {
                if (s.charAt(i - 1) == ' ') return false;
                continue;
            }
            return false;
        }
        return true;
    }

    public static void main(String[] args) {
        String stream = "Hello world. this is bad.";
        StringBuilder buf = new StringBuilder();
        for (int k = 0; k < stream.length(); k++) {
            char ch = stream.charAt(k);
            buf.append(ch);
            if (isTerminal(ch)) {
                String candidate = buf.toString().trim();
                if (isValid(candidate)) System.out.println(candidate);
                buf.setLength(0);
            }
        }
    }
}
