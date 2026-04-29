// Day 1441: Validate UTF-8 encoding from an array of byte values.
// Approach: scan bytes, read leading-one count of the lead byte, verify
// continuation bytes start with 10. Time O(n), Space O(1).
public class Solution {
    static boolean validUtf8(int[] data) {
        int i = 0, n = data.length;
        while (i < n) {
            int b = data[i] & 0xFF;
            int cnt;
            if ((b >> 7) == 0b0) cnt = 1;
            else if ((b >> 5) == 0b110) cnt = 2;
            else if ((b >> 4) == 0b1110) cnt = 3;
            else if ((b >> 3) == 0b11110) cnt = 4;
            else return false;
            if (i + cnt > n) return false;
            for (int j = 1; j < cnt; j++) {
                int c = data[i + j] & 0xFF;
                if ((c >> 6) != 0b10) return false;
            }
            i += cnt;
        }
        return true;
    }

    public static void main(String[] args) {
        int[] euro = {226, 130, 172}; // 11100010 10000010 10101100
        System.out.println(validUtf8(euro) ? "True" : "False");
    }
}
