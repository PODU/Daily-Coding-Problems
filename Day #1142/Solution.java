// Day 1142: nth sevenish number = sum of distinct powers of 7.
// Bits of n in binary select powers of 7 (bijection). Time O(log n), Space O(1).
public class Solution {
    static long sevenish(long n) {
        long result = 0, power = 1;
        while (n > 0) {
            if ((n & 1) == 1) result += power;
            power *= 7;
            n >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        for (long i = 1; i <= 5; i++) System.out.print(sevenish(i) + " "); // 1 7 8 49 50
        System.out.println();
    }
}
