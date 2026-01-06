// Day 864: Minimum rescue boats (<=2 people, total weight <= k).
// Approach: sort, greedily pair lightest with heaviest using two pointers.
// Time: O(n log n), Space: O(1).
import java.util.*;

public class Solution {
    static int numRescueBoats(int[] w, int k) {
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
        System.out.println(numRescueBoats(new int[]{100, 200, 150, 80}, 200)); // 3
    }
}
