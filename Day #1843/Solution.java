// Day 1843: Max of two numbers with no branching/comparison; uses sign bit of the difference.
// max(a,b) = a - ((a-b) & ((a-b) >> 63)). Time O(1), Space O(1).
public class Solution {
    static long maxNoBranch(long a, long b) {
        long d = a - b;
        return a - (d & (d >> 63));
    }

    public static void main(String[] args) {
        System.out.println(maxNoBranch(5, 9));   // 9
        System.out.println(maxNoBranch(12, 4));  // 12
        System.out.println(maxNoBranch(-3, -7)); // -3
    }
}
