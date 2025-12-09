// Day 722: Minimum swaps so each couple sits side by side.
// Approach: Greedy - fix each even seat; partner = x^1. Each swap places one couple.
// Answer equals N - (#cycles). Time: O(N), Space: O(N).
import java.util.*;

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
                pos[partner] = i + 1;
                int t = row[i + 1]; row[i + 1] = row[j]; row[j] = t;
                swaps++;
            }
        }
        return swaps;
    }

    public static void main(String[] args) {
        int[] row = {0, 2, 1, 3};
        System.out.println("Minimum swaps: " + minSwaps(row)); // 1
    }
}
