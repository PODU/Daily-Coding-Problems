// Day 1021: Swap even and odd bits of an 8-bit integer.
// Approach: ((n & 0xAA) >> 1) | ((n & 0x55) << 1).  Time O(1), Space O(1).
public class Solution {
    static int swapBits(int n) { return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF; }

    static String toBin(int n) {
        StringBuilder s = new StringBuilder();
        for (int i = 7; i >= 0; --i) s.append((n >> i) & 1);
        return s.toString();
    }

    public static void main(String[] args) {
        String[] in = {"10101010", "11100010"};
        for (String b : in) {
            int n = Integer.parseInt(b, 2);
            System.out.println(toBin(swapBits(n)));
        }
    }
}
