// Day 277: Validate UTF-8 from byte-value array. Single pass.
// Time O(N), Space O(1). Only low 8 bits of each integer are used.
public class Solution {
    static boolean validUTF8(int[] data) {
        int remaining = 0;
        for (int v : data) {
            int b = v & 0xFF;
            if (remaining == 0) {
                if ((b >> 7) == 0) remaining = 0;
                else if ((b >> 5) == 0b110) remaining = 1;
                else if ((b >> 4) == 0b1110) remaining = 2;
                else if ((b >> 3) == 0b11110) remaining = 3;
                else return false;
            } else {
                if ((b >> 6) != 0b10) return false;
                remaining--;
            }
        }
        return remaining == 0;
    }

    public static void main(String[] args) {
        System.out.println(validUTF8(new int[]{0b11100010, 0b10000010, 0b10101100})); // true
        System.out.println(validUTF8(new int[]{0b11101011, 0b10001100, 0b00000100})); // false
    }
}
