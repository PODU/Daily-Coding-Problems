// Max of two numbers without if-else/branch/comparison via bit manipulation.
// d=a-b; mask = d>>63 (arithmetic sign extend); max = a - (d & mask). Time O(1), Space O(1).
public class Solution {
    static long maxNoBranch(long a, long b) {
        long d = a - b;
        long mask = d >> 63; // all 1s if d<0, else 0
        return a - (d & mask);
    }

    public static void main(String[] args) {
        System.out.println(maxNoBranch(3, 7));
        System.out.println(maxNoBranch(10, -4));
    }
}
