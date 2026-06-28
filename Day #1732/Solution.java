// Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
public class Solution {
    static int rangeBitwiseAnd(int m, int n) {
        int shift = 0;
        while (m < n) { m >>= 1; n >>= 1; shift++; }
        return m << shift;
    }

    public static void main(String[] args) {
        System.out.println(rangeBitwiseAnd(5, 7));
    }
}
