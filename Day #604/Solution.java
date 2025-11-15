// Day 604: Soundex phonetic encoding (letter + 3 digits).
// Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).
public class Solution {
    // '1'..'6' consonant codes, '0' vowel separator, 'S' transparent (h, w).
    static char code(char c) {
        switch (c) {
            case 'B': case 'F': case 'P': case 'V': return '1';
            case 'C': case 'G': case 'J': case 'K': case 'Q': case 'S': case 'X': case 'Z': return '2';
            case 'D': case 'T': return '3';
            case 'L': return '4';
            case 'M': case 'N': return '5';
            case 'R': return '6';
            case 'H': case 'W': return 'S';
            default: return '0';
        }
    }

    static String soundex(String name) {
        StringBuilder up = new StringBuilder();
        for (char c : name.toCharArray())
            if (Character.isLetter(c)) up.append(Character.toUpperCase(c));
        if (up.length() == 0) return "0000";
        StringBuilder res = new StringBuilder();
        res.append(up.charAt(0));
        char prev = code(up.charAt(0));
        for (int i = 1; i < up.length() && res.length() < 4; i++) {
            char c = code(up.charAt(i));
            if (c >= '1' && c <= '6') {
                if (c != prev) res.append(c);
                prev = c;
            } else if (c == '0') {
                prev = '0';
            }
        }
        while (res.length() < 4) res.append('0');
        return res.substring(0, 4);
    }

    public static void main(String[] args) {
        System.out.println(soundex("Jackson")); // J250
        System.out.println(soundex("Jaxen"));   // J250
    }
}
