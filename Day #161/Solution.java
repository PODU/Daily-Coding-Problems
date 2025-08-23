// Day 161: Reverse the 32 bits of an integer by shifting bits out of the input
// into the result. Time O(32), Space O(1).
public class Solution {
    static int reverseBits(int n) {
        int res = 0;
        for (int i = 0; i < 32; i++) {
            res = (res << 1) | (n & 1);
            n >>>= 1;
        }
        return res;
    }

    static String toGroups(int n) {
        StringBuilder sb = new StringBuilder();
        for (int i = 31; i >= 0; i--) {
            sb.append(((n >>> i) & 1) == 1 ? '1' : '0');
            if (i % 4 == 0 && i != 0) sb.append(' ');
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        int n = 0xF0F0F0F0; // 1111 0000 ... repeated
        System.out.println(toGroups(reverseBits(n)));
    }
}
