// Day 1424: URL shortener (shorten -> 6-char code, restore -> original or null).
// Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.
import java.util.*;

public class Solution {
    static final String ALPHABET =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    long counter = 0;
    Map<String,String> urlToShort = new HashMap<>();
    Map<String,String> shortToUrl = new HashMap<>();

    String encode(long n) {
        char[] s = new char[6];
        for (int i = 5; i >= 0; i--) { s[i] = ALPHABET.charAt((int)(n % 62)); n /= 62; }
        return new String(s);
    }

    String shorten(String url) {
        if (urlToShort.containsKey(url)) return urlToShort.get(url); // same URL -> same code
        String code = encode(counter++);
        urlToShort.put(url, code);
        shortToUrl.put(code, url);
        return code;
    }

    String restore(String code) {
        return shortToUrl.getOrDefault(code, "null");
    }

    public static void main(String[] args) {
        Solution s = new Solution();
        String a = s.shorten("https://example.com/page");
        String b = s.shorten("https://example.com/page");
        System.out.println(a);                       // 000000
        System.out.println(a.equals(b) ? "true" : "false"); // true
        System.out.println(s.restore(a));            // https://example.com/page
        System.out.println(s.restore("zzzzzz"));     // null
    }
}
