// URL shortener: base62 6-char codes from a counter, two hash maps + reverse map for dedupe.
// Time O(1) per shorten/restore, space O(n).
import java.util.*;

public class Solution {
    static final String ALPHABET =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    long counter;
    Map<String,String> codeToUrl = new HashMap<>();
    Map<String,String> urlToCode = new HashMap<>();

    Solution() { counter = decode("abcdef"); }

    String encode(long num) {
        char[] s = new char[6];
        for (int i = 5; i >= 0; i--) { s[i] = ALPHABET.charAt((int)(num % 62)); num /= 62; }
        return new String(s);
    }
    long decode(String s) {
        long num = 0;
        for (int i = 0; i < s.length(); i++) num = num * 62 + ALPHABET.indexOf(s.charAt(i));
        return num;
    }
    String shorten(String url) {
        if (urlToCode.containsKey(url)) return urlToCode.get(url);
        String code = encode(counter++);
        codeToUrl.put(code, url);
        urlToCode.put(url, code);
        return code;
    }
    String restore(String code) { return codeToUrl.get(code); }

    public static void main(String[] args) {
        Solution s = new Solution();
        String code = s.shorten("https://www.example.com/some/long/path");
        System.out.println(code);
        String r1 = s.restore(code);
        System.out.println(r1 == null ? "null" : r1);
        String r2 = s.restore("XXXXXX");
        System.out.println(r2 == null ? "null" : r2);
    }
}
