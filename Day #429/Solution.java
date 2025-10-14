// Day 429: kth row of Pascal's triangle (0-indexed: row 0 = [1]).
// Multiplicative recurrence row[j] = row[j-1]*(k-j+1)/j. Time O(k), Space O(k).
public class Solution {
    public static void main(String[] args) {
        int k = 4;
        long[] row = new long[k + 1];
        row[0] = 1;
        for (int j = 1; j <= k; j++) row[j] = row[j - 1] * (k - j + 1) / j;
        StringBuilder sb = new StringBuilder();
        for (int j = 0; j <= k; j++) {
            sb.append(row[j]);
            if (j < k) sb.append(" ");
        }
        System.out.println(sb.toString());
    }
}
