// Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
// dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
import java.util.*;

public class Solution {
    static class Shortener {
        static final String A = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        Map<String, String> code2url = new HashMap<>(), url2code = new HashMap<>();
        long counter = 916132831L;

        String encode(long n) {
            char[] s = new char[6];
            for (int i = 5; i >= 0; i--) { s[i] = A.charAt((int)(n % 62)); n /= 62; }
            return new String(s);
        }
        String shorten(String url) {
            if (url2code.containsKey(url)) return url2code.get(url);
            String code = encode(counter++);
            code2url.put(code, url); url2code.put(url, code);
            return code;
        }
        String restore(String code) { return code2url.getOrDefault(code, null); }
    }

    public static void main(String[] args) {
        Shortener s = new Shortener();
        String c = s.shorten("https://example.com/long/path");
        System.out.println("short: " + c);
        System.out.println("restore: " + s.restore(c));
        System.out.println("restore(zzzzzz): " + s.restore("zzzzzz")); // null
    }
}
