// Day 109: Swap adjacent bit pairs: ((x&0xAA)>>1)|((x&0x55)<<1). O(1).
public class Solution {
    static int swapBits(int x){
        return (((x & 0xAA) >> 1) | ((x & 0x55) << 1)) & 0xFF;
    }
    static String toBin(int x){
        return String.format("%8s", Integer.toBinaryString(x)).replace(' ', '0');
    }
    public static void main(String[] args){
        System.out.println(toBin(swapBits(0b10101010))); // 01010101
        System.out.println(toBin(swapBits(0b11100010))); // 11010001
    }
}
