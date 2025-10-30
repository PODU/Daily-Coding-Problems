// Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
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
        // First few sevenish numbers: 1, 7, 8, 49, ...
        StringBuilder sb = new StringBuilder();
        for (int n = 1; n <= 4; n++) {
            sb.append(sevenish(n));
            if (n < 4) sb.append(", ");
        }
        System.out.println(sb);
    }
}
