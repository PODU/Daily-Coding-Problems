// Day 719: Convert 1-based column number to spreadsheet id (bijective base-26).
// Repeatedly take (n-1)%26 then n=(n-1)/26. Time O(log n).
public class Solution {
    static String colId(long n) {
        StringBuilder sb = new StringBuilder();
        while (n > 0) {
            n--;
            sb.append((char) ('A' + n % 26));
            n /= 26;
        }
        return sb.reverse().toString();
    }

    public static void main(String[] args) {
        System.out.println("\"" + colId(1) + "\"");
        System.out.println("\"" + colId(27) + "\"");
    }
}
