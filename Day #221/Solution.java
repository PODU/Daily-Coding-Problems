// Day 221: nth "sevenish" number (sum of distinct powers of 7).
// Approach: bits of n select which powers of 7 to include (bijection with binary). Time O(log n), Space O(1).
public class Solution {
    static long sevenish(long n) {
        long result = 0, power = 1; // 7^0
        while (n > 0) {
            if ((n & 1) == 1) result += power;
            power *= 7;
            n >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i <= 5; i++) sb.append(sevenish(i)).append(i < 5 ? " " : "");
        System.out.println(sb); // 1 7 8 49 50
        System.out.println(sevenish(4)); // 49
    }
}
