// Longest consecutive run of 1-bits via the bit trick: n &= (n>>1) shrinks every
// run by one each step; iterations until n==0 equals the longest run length.
// Time: O(longest run), Space: O(1).
public class Solution {
    static int longestRun(int n) {
        int count = 0;
        while (n != 0) {
            count++;
            n &= (n >>> 1);
        }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(longestRun(156)); // 156 = 10011100 -> 3
    }
}
