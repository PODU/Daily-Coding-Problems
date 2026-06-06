// Sevenish: write n in binary; each set bit k contributes 7^k. Time O(log n), Space O(1).
public class Solution {
    static long sevenish(long n) {
        long sum = 0, pow7 = 1;
        while (n > 0) {
            if ((n & 1) == 1) sum += pow7;
            pow7 *= 7;
            n >>= 1;
        }
        return sum;
    }

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        for (int n = 1; n <= 5; n++) {
            sb.append(sevenish(n));
            if (n < 5) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
