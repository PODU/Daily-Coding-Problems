// Iterate integers, sum digits, count until the n-th whose digit sum is 10.
// Time O(answer * digits), Space O(1).
public class Solution {
    static int digitSum(long x) {
        int s = 0;
        while (x > 0) { s += x % 10; x /= 10; }
        return s;
    }

    static long nthPerfect(int n) {
        int count = 0;
        long num = 0;
        while (count < n) {
            num++;
            if (digitSum(num) == 10) count++;
        }
        return num;
    }

    public static void main(String[] args) {
        System.out.println(nthPerfect(1));
    }
}
