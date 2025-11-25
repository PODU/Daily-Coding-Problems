// Reverse 32 bits: iterate 32 times, shift result left, OR in LSB of input, logical-shift input right.
// Time O(32)=O(1), space O(1). Uses long & 0xFFFFFFFFL for unsigned handling.
public class Solution {
    static long reverseBits(long x) {
        x &= 0xFFFFFFFFL;
        long result = 0;
        for (int i = 0; i < 32; i++) {
            result = (result << 1) | (x & 1L);
            x >>= 1;
        }
        return result & 0xFFFFFFFFL;
    }

    public static void main(String[] args) {
        long input = 0xF0F0F0F0L;
        long out = reverseBits(input);
        StringBuilder sb = new StringBuilder();
        for (int i = 31; i >= 0; i--) {
            sb.append(((out >> i) & 1L) == 1 ? '1' : '0');
            if (i % 4 == 0 && i != 0) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
