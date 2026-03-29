// Day 1280: Swap adjacent bit pairs of an 8-bit unsigned integer.
// One-liner: shift odd bits up, even bits down. Time O(1), Space O(1).
public class Solution {
    static int swapBits(int n) {
        return ((n & 0xAA) >> 1) | ((n & 0x55) << 1);
    }

    static String toBin(int n) {
        String s = Integer.toBinaryString(n & 0xFF);
        return "0".repeat(8 - s.length()) + s;
    }

    public static void main(String[] args) {
        for (String in : new String[]{"10101010", "11100010"}) {
            int n = Integer.parseInt(in, 2);
            System.out.println(toBin(swapBits(n)));
        }
    }
}
