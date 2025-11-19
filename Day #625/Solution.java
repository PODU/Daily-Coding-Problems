// Longest consecutive run of 1s in binary repr of n.
// Bit trick: n &= (n<<1) collapses runs; iterations = longest run. O(bits) time, O(1) space.
public class Solution {
    static int longestRun(int n) {
        int count = 0;
        while (n != 0) { n &= (n << 1); count++; }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(longestRun(156)); // 10011100 -> 3
    }
}
