// Reverse all 32 bits of an int by shifting LSB of input into LSB-first of result.
// Time: O(1) (32 steps); Space: O(1).
public class Solution {
    static int reverseBits(int x) {
        int r = 0;
        for (int i = 0; i < 32; i++) {
            r = (r << 1) | (x & 1);
            x >>>= 1; // unsigned shift
        }
        return r;
    }

    static String toGrouped(int x) {
        StringBuilder s = new StringBuilder();
        for (int i = 31; i >= 0; i--) {
            s.append(((x >>> i) & 1) == 1 ? '1' : '0');
            if (i % 4 == 0 && i != 0) s.append(' ');
        }
        return s.toString();
    }

    public static void main(String[] args) {
        int input = 0xF0F0F0F0;
        System.out.println("Input:  " + toGrouped(input));
        System.out.println(toGrouped(reverseBits(input)));
    }
}
