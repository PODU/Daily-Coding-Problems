// URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
// Dedup via url->code map so identical URLs map to the same code.
// shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.
import java.util.*;

public class Solution {
    static final String ALPHA =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    private long counter = 0;
    private final Map<String, String> urlToCode = new HashMap<>();
    private final Map<String, String> codeToUrl = new HashMap<>();

    private String encode(long n) {
        char[] s = new char[6];
        Arrays.fill(s, ALPHA.charAt(0));
        int i = 5;
        while (n > 0 && i >= 0) {
            s[i--] = ALPHA.charAt((int) (n % 62));
            n /= 62;
        }
        return new String(s);
    }

    public String shorten(String url) {
        if (urlToCode.containsKey(url)) return urlToCode.get(url); // same URL -> same code
        String code = encode(counter++);
        urlToCode.put(url, code);
        codeToUrl.put(code, url);
        return code;
    }

    public String restore(String code) {
        return codeToUrl.get(code); // null if unknown
    }

    public static void main(String[] args) {
        Solution s = new Solution();
        String url = "https://www.example.com/some/long/path";
        String code = s.shorten(url);
        System.out.println("shorten(" + url + ") = " + code);
        System.out.println("restore(" + code + ") = " + s.restore(code));
        System.out.println("restore(zzzzzz) = " + s.restore("zzzzzz"));
        String code2 = s.shorten(url);
        System.out.println("shorten same url again = " + code2
            + " (same as before: " + code.equals(code2) + ")");
    }
}
