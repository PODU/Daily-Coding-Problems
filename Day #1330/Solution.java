// Day 1330: Column number -> spreadsheet label (bijective base-26).
// Repeatedly take (n-1)%26 for the next letter, then n=(n-1)/26. O(log n) time.
public class Solution {
    static String columnTitle(long n) {
        StringBuilder sb = new StringBuilder();
        while (n > 0) {
            n--;
            sb.append((char) ('A' + n % 26));
            n /= 26;
        }
        return sb.reverse().toString();
    }

    public static void main(String[] args) {
        System.out.println("\"" + columnTitle(1) + "\"");  // "A"
        System.out.println("\"" + columnTitle(27) + "\""); // "AA"
    }
}
