// Branchless max: use 64-bit diff to avoid overflow; sign bit via arithmetic
// right shift selects the larger. No if/branch/compare. Time O(1), Space O(1).
public class Solution {
    static int maxNoBranch(int a, int b) {
        long diff = (long) a - (long) b;      // a - b without int overflow
        long sign = diff >> 63;               // -1 if diff<0 else 0
        return (int) (a - (diff & sign));     // a>=b -> a ; a<b -> b
    }

    public static void main(String[] args) {
        System.out.println("max(3, 7) = " + maxNoBranch(3, 7));
        System.out.println("max(10, -5) = " + maxNoBranch(10, -5));
    }
}
