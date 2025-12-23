// Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.
public class Solution {
    static int nextSamePopcount(int n) {
        if (n == 0) return 0;
        int c = n & -n;                          // lowest set bit
        int r = n + c;                           // ripple carry
        int ones = ((n ^ r) >>> 2) / c;          // shifted-in ones
        return r | ones;
    }

    public static void main(String[] args) {
        System.out.println(nextSamePopcount(6)); // 0110 -> 1001 = 9
    }
}
