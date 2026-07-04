// Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
// collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
// Time: O(n) per name, Space: O(n).
public class Solution {
    static int code(char c) {
        c = Character.toLowerCase(c);
        switch (c) {
            case 'b': case 'f': case 'p': case 'v': return 1;
            case 'c': case 'g': case 'j': case 'k': case 'q':
            case 's': case 'x': case 'z': return 2;
            case 'd': case 't': return 3;
            case 'l': return 4;
            case 'm': case 'n': return 5;
            case 'r': return 6;
            default: return 0; // vowels a,e,i,o,u,y and h,w
        }
    }

    static String soundex(String name) {
        StringBuilder s = new StringBuilder();
        for (char c : name.toCharArray()) if (Character.isLetter(c)) s.append(c);
        if (s.length() == 0) return "";
        StringBuilder res = new StringBuilder();
        res.append(Character.toUpperCase(s.charAt(0)));
        int prev = code(s.charAt(0));
        for (int i = 1; i < s.length() && res.length() < 4; i++) {
            char ch = s.charAt(i);
            int d = code(ch);
            char lc = Character.toLowerCase(ch);
            if (d != 0 && d != prev) res.append((char) ('0' + d));
            if (lc != 'h' && lc != 'w') prev = d;
        }
        while (res.length() < 4) res.append('0');
        return res.substring(0, 4);
    }

    public static void main(String[] args) {
        System.out.println(soundex("Jackson"));
        System.out.println(soundex("Jaxen"));
    }
}
