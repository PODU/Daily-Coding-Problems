// Day 420: n-th positive integer whose digits sum to exactly 10.
// Iterate integers, count those with digit sum 10. Time: O(answer), Space: O(1).
public class Solution {
    static int digitSum(long x) {
        int s = 0;
        while (x > 0) { s += x % 10; x /= 10; }
        return s;
    }

    static long nthPerfect(int n) {
        int count = 0;
        long x = 0;
        while (count < n) {
            x++;
            if (digitSum(x) == 10) count++;
        }
        return x;
    }

    public static void main(String[] args) {
        System.out.println(nthPerfect(1)); // 19
        System.out.println(nthPerfect(2)); // 28
    }
}
