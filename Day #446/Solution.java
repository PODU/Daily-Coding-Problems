// Day 446: Power of four test in O(1) (better than O(log N)).
// Power of two (single set bit) AND that bit sits at an even position.
public class Solution {
    static boolean isPowerOfFour(int n) {
        return n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0;
    }

    public static void main(String[] args) {
        int[] tests = {1, 4, 8, 16, 64, 0, 5, 256};
        for (int n : tests)
            System.out.println(n + " -> " + isPowerOfFour(n));
        // 1->true 4->true 8->false 16->true 64->true 0->false 5->false 256->true
    }
}
