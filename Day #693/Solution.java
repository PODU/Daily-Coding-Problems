// Day 693: Swap adjacent (even/odd) bits of an unsigned 8-bit integer.
// Approach: one-liner masks. Odd bits 0x55 shift left, even bits 0xAA shift right.
// Time O(1), Space O(1).
public class Solution {
    static int swapBits(int n) {
        return (((n & 0xAA) >> 1) | ((n & 0x55) << 1)) & 0xFF;
    }

    static String bin8(int n) {
        return String.format("%8s", Integer.toBinaryString(n)).replace(' ', '0');
    }

    public static void main(String[] args) {
        System.out.println(bin8(swapBits(0b10101010))); // 01010101
        System.out.println(bin8(swapBits(0b11100010))); // 11010001
    }
}
