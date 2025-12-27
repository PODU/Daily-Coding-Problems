// Day 805: Spreadsheet column number -> alphabetical label (bijective base 26).
// Repeatedly take (n-1)%26 for the letter, then n=(n-1)/26. Time O(log n), Space O(log n).
public class Solution {
    static String columnLabel(long n) {
        StringBuilder sb = new StringBuilder();
        while (n > 0) {
            n--;
            sb.append((char) ('A' + n % 26));
            n /= 26;
        }
        return sb.reverse().toString();
    }

    public static void main(String[] args) {
        System.out.println("\"" + columnLabel(1) + "\"");  // "A"
        System.out.println("\"" + columnLabel(27) + "\""); // "AA"
    }
}
