// UTF-8 validation: read lead byte, count leading ones (1->1byte, 2..4 multi), verify continuation bytes start with 10.
// Time O(n), Space O(1).
public class Solution {
    static boolean validUtf8(int[] data) {
        int i = 0, n = data.length;
        while (i < n) {
            int b = data[i] & 0xFF;
            int cnt;
            if ((b & 0x80) == 0x00) cnt = 1;
            else if ((b & 0xE0) == 0xC0) cnt = 2;
            else if ((b & 0xF0) == 0xE0) cnt = 3;
            else if ((b & 0xF8) == 0xF0) cnt = 4;
            else return false;
            if (i + cnt > n) return false;
            for (int k = 1; k < cnt; k++) {
                int c = data[i + k] & 0xFF;
                if ((c & 0xC0) != 0x80) return false;
            }
            i += cnt;
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(validUtf8(new int[]{226, 130, 172}));          // true
        System.out.println(validUtf8(new int[]{0b11110101, 0b10000010, 0b00000010})); // false
    }
}
