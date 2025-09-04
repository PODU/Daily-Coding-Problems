// Day 214: Longest consecutive run of 1s in binary representation.
// Approach: n &= (n<<1) collapses runs; count iterations. Time O(longest run), Space O(1).
public class Solution {
    static int longestRun(long n) {
        int count = 0;
        while (n != 0) {
            n &= (n << 1);
            count++;
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(longestRun(156)); // 156 = 10011100 -> 3
    }
}
