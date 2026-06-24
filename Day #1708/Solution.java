// Power of four: N>0 && single set bit (N&(N-1))==0 && bit at even position (N & 0x55555555). O(1).
public class Solution {
    static boolean isPowerOfFour(int n) {
        return n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0;
    }

    public static void main(String[] args) {
        int[] tests = {16, 8, 1, 64, 5};
        for (int t : tests)
            System.out.println(t + " -> " + isPowerOfFour(t));
    }
}
