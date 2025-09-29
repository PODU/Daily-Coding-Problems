// Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
// (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
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
            default: return 0; // vowels, y, w, h
        }
    }

    static String soundex(String name) {
        StringBuilder res = new StringBuilder();
        res.append(Character.toUpperCase(name.charAt(0)));
        int prev = code(name.charAt(0));
        for (int i = 1; i < name.length() && res.length() < 4; i++) {
            char c = Character.toLowerCase(name.charAt(i));
            int d = code(c);
            if (d != 0 && d != prev) res.append((char) ('0' + d));
            if (c == 'h' || c == 'w') continue; // transparent: keep prev
            prev = d;                            // vowels reset prev to 0
        }
        while (res.length() < 4) res.append('0');
        return res.toString();
    }

    public static void main(String[] args) {
        System.out.println(soundex("Jackson"));
    }
}
