// Soundex phonetic encoding: keep first letter, map consonants to digits,
// collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.
public class Solution {
    static int code(char c) {
        switch (Character.toLowerCase(c)) {
            case 'b': case 'f': case 'p': case 'v': return 1;
            case 'c': case 'g': case 'j': case 'k': case 'q':
            case 's': case 'x': case 'z': return 2;
            case 'd': case 't': return 3;
            case 'l': return 4;
            case 'm': case 'n': return 5;
            case 'r': return 6;
            default: return 0;
        }
    }

    static String soundex(String name) {
        if (name.isEmpty()) return "";
        StringBuilder out = new StringBuilder();
        out.append(Character.toUpperCase(name.charAt(0)));
        int prev = code(name.charAt(0));
        for (int i = 1; i < name.length() && out.length() < 4; i++) {
            char ch = Character.toLowerCase(name.charAt(i));
            int c = code(ch);
            if (c != 0 && c != prev) out.append((char) ('0' + c));
            if (ch == 'h' || ch == 'w') continue;
            prev = c;
        }
        while (out.length() < 4) out.append('0');
        return out.substring(0, 4);
    }

    public static void main(String[] args) {
        System.out.println(soundex("Jackson")); // J250
        System.out.println(soundex("Jaxen"));   // J250
    }
}
