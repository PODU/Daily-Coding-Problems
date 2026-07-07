// Swap even/odd bits of 8-bit int: ((n&0xAA)>>1)|((n&0x55)<<1), masked to 8 bits.
// Time: O(1), Space: O(1).
public class Solution {
    static String swapBits(String bin) {
        int n = Integer.parseInt(bin, 2);
        int r = (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
        StringBuilder sb = new StringBuilder();
        for (int i = 7; i >= 0; --i) sb.append(((r >> i) & 1) == 1 ? '1' : '0');
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(swapBits("10101010"));
        System.out.println(swapBits("11100010"));
    }
}
