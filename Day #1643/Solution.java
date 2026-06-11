// Reverse the 32 bits of a 32-bit integer by shifting result left and pulling
// the lowest bit of the input each iteration. Time O(1) (32 iters), Space O(1).
public class Solution {
    static int reverseBits(int n) {
        int result = 0;
        for (int i = 0; i < 32; i++) {
            result = (result << 1) | (n & 1);
            n >>>= 1;
        }
        return result;
    }

    static String groupNibbles(int n) {
        StringBuilder sb = new StringBuilder();
        for (int i = 31; i >= 0; i--) {
            sb.append(((n >>> i) & 1) == 1 ? '1' : '0');
            if (i % 4 == 0 && i != 0) sb.append(' ');
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        int input = 0xF0F0F0F0;
        int rev = reverseBits(input);
        System.out.println(groupNibbles(rev));
    }
}
