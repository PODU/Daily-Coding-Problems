// Day 716: Validate UTF-8. Read leading byte to get char length (1-4) from its
// high bits, then verify each continuation byte starts with 10. Time O(n).
public class Solution {
    static boolean validUtf8(int[] data) {
        int i = 0, n = data.length;
        while (i < n) {
            int b = data[i] & 0xFF, len;
            if ((b >> 7) == 0b0) len = 1;
            else if ((b >> 5) == 0b110) len = 2;
            else if ((b >> 4) == 0b1110) len = 3;
            else if ((b >> 3) == 0b11110) len = 4;
            else return false;
            if (i + len > n) return false;
            for (int j = 1; j < len; j++)
                if (((data[i + j] & 0xFF) >> 6) != 0b10) return false;
            i += len;
        }
        return true;
    }

    public static void main(String[] args) {
        System.out.println(validUtf8(new int[]{226, 130, 172}) ? "True" : "False");
        System.out.println(validUtf8(new int[]{235, 140, 4}) ? "True" : "False");
    }
}
