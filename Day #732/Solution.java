// Day 732: Minimum boats (each holds <=2 people, weight limit k).
// Approach: Sort; two pointers pair lightest with heaviest when they fit.
// Time: O(n log n), Space: O(1).
import java.util.*;

public class Solution {
    static int numBoats(int[] w, int k) {
        Arrays.sort(w);
        int i = 0, j = w.length - 1, boats = 0;
        while (i <= j) {
            if (w[i] + w[j] <= k) i++;
            j--;
            boats++;
        }
        return boats;
    }

    public static void main(String[] args) {
        int[] weights = {100, 200, 150, 80};
        System.out.println(numBoats(weights, 200)); // 3
    }
}
