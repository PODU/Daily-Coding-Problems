// Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
// Time: O(1) amortized per op, Space: O(n).
import java.util.*;

public class Solution {
    static class URLShortener {
        private static final String ALPHA =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        private final Map<String, String> toLong = new HashMap<>();
        private final Map<String, String> toShort = new HashMap<>();
        private final Random rng = new Random();

        private String randCode() {
            StringBuilder sb = new StringBuilder(6);
            for (int i = 0; i < 6; i++) sb.append(ALPHA.charAt(rng.nextInt(ALPHA.length())));
            return sb.toString();
        }

        String shorten(String url) {
            if (toShort.containsKey(url)) return toShort.get(url); // same URL -> same code
            String code;
            do { code = randCode(); } while (toLong.containsKey(code));
            toLong.put(code, url);
            toShort.put(url, code);
            return code;
        }

        // Returns original URL, or null if the code is unknown.
        String restore(String code) { return toLong.get(code); }
    }

    public static void main(String[] args) {
        URLShortener s = new URLShortener();
        String a = s.shorten("https://example.com/foo");
        String b = s.shorten("https://example.com/foo"); // same URL twice
        System.out.println("same code reused: " + a.equals(b));
        System.out.println("restore: " + s.restore(a));
        System.out.println("restore unknown: " + s.restore("zzzzzz"));
    }
}
