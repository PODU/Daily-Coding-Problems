// n-th perfect number (digits sum to 10): count up from 1 checking digit sums. Time O(answer * digits), Space O(1).
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
        System.out.println(nthPerfect(2));
    }
}
