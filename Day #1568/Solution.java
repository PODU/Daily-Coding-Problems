// Longest consecutive run of 1s using n &= (n<<1) bit trick. Time O(run length), space O(1).
public class Solution {
    static int longestRun(long n) {
        int count = 0;
        while (n != 0) { n &= (n << 1); count++; }
        return count;
    }

    public static void main(String[] args) {
        System.out.println(longestRun(156)); // 10011100 -> 3
    }
}
