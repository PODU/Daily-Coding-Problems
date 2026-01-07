// Soundex phonetic encoding: keep first letter, code rest, dedupe, pad to 3 digits.
// Time O(n), Space O(1) extra per name.
public class Solution {
    static int code(char c){
        c = Character.toLowerCase(c);
        switch(c){
            case 'b': case 'f': case 'p': case 'v': return 1;
            case 'c': case 'g': case 'j': case 'k': case 'q':
            case 's': case 'x': case 'z': return 2;
            case 'd': case 't': return 3;
            case 'l': return 4;
            case 'm': case 'n': return 5;
            case 'r': return 6;
            default:  return 0; // vowels a,e,i,o,u,y
        }
    }

    static String soundex(String name){
        int i = 0, n = name.length();
        while(i < n && !Character.isLetter(name.charAt(i))) i++;
        if(i >= n) return "";
        StringBuilder res = new StringBuilder();
        res.append(Character.toUpperCase(name.charAt(i)));
        int prev = code(name.charAt(i));
        for(int j = i + 1; j < n && res.length() < 4; j++){
            char c = Character.toLowerCase(name.charAt(j));
            if(!Character.isLetter(c)) continue;
            if(c == 'h' || c == 'w') continue;
            int d = code(c);
            if(d == 0){ prev = 0; continue; }
            if(d != prev) res.append((char)('0' + d));
            prev = d;
        }
        while(res.length() < 4) res.append('0');
        return res.substring(0, 4);
    }

    public static void main(String[] args){
        System.out.println(soundex("Jackson"));
        System.out.println(soundex("Jaxen"));
    }
}
