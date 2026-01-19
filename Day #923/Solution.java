// Power of four via O(1) bit manipulation.
// power of two: n>0 && (n&(n-1))==0; set bit in even position: (n & 0x55555555)!=0.
// Time O(1), Space O(1).
public class Solution {
    static boolean isPowerOfFour(int n) {
        return n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0;
    }

    public static void main(String[] args) {
        int[] tests = {1, 4, 16, 64, 8, 5, 0};
        for (int n : tests)
            System.out.println(n + ": " + isPowerOfFour(n));
    }
}
