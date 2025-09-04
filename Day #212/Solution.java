// Day 212: Spreadsheet column number -> label (bijective base-26).
// Approach: repeatedly take (n-1)%26 for the digit, then n=(n-1)/26. Time O(log n), Space O(log n).
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
        System.out.println("\"" + columnLabel(1) + "\"");
        System.out.println("\"" + columnLabel(27) + "\"");
    }
}
