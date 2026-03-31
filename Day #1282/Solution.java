// Day 1282: n-th positive integer whose digits sum to 10.
// Such numbers are ~ every 9th integer; iterate counting matches. Time O(answer/9), Space O(1).
public class Solution {
    static int digitSum(long x) {
        int s = 0;
        while (x > 0) { s += x % 10; x /= 10; }
        return s;
    }

    static long nthPerfect(int n) {
        long x = 0;
        int count = 0;
        while (count < n) {
            ++x;
            if (digitSum(x) == 10) ++count;
        }
        return x;
    }

    public static void main(String[] args) {
        System.out.println(nthPerfect(1)); // 19
        System.out.println(nthPerfect(2)); // 28
    }
}
