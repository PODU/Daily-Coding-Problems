// Day 1202: Minimum swaps so couples sit side by side.
// Greedy: for each even seat, swap partner of its occupant into the next seat. Time O(N), Space O(N).
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
                int t = pos[row[i + 1]]; pos[row[i + 1]] = pos[row[j]]; pos[row[j]] = t;
                t = row[i + 1]; row[i + 1] = row[j]; row[j] = t;
                swaps++;
            }
        }
        return swaps;
    }

    public static void main(String[] args) {
        System.out.println(minSwaps(new int[]{0, 2, 1, 3})); // 1
    }
}
