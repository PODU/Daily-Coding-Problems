// Minimum swaps to seat N couples side by side.
// Greedy: partner of p is p^1; swap mismatched partner into place. Time: O(N), Space: O(N).

public class Solution {
    static int minSwaps(int[] row) {
        int n = row.length;
        int[] pos = new int[n];
        for (int i = 0; i < n; i++) pos[row[i]] = i;
        int swaps = 0;
        for (int i = 0; i < n; i += 2) {
            int partner = row[i] ^ 1;
            if (row[i + 1] != partner) {
                int j = pos[partner];
                pos[row[i + 1]] = j;
                pos[row[j]] = i + 1;
                int tmp = row[i + 1];
                row[i + 1] = row[j];
                row[j] = tmp;
                swaps++;
            }
        }
        return swaps;
    }

    public static void main(String[] args) {
        int[] row = {0, 2, 1, 3};
        System.out.println(minSwaps(row));
    }
}
