// Boat rescue: min boats, <=2 people each, weight limit k.
// Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.
import java.util.Arrays;

public class Solution {
    static int numRescueBoats(int[] w, int k) {
        Arrays.sort(w);
        int lo = 0, hi = w.length - 1, boats = 0;
        while (lo <= hi) {
            if (w[lo] + w[hi] <= k) lo++;
            hi--;
            boats++;
        }
        return boats;
    }

    public static void main(String[] args) {
        int[] w = {100, 200, 150, 80};
        System.out.println(numRescueBoats(w, 200)); // expected 3
    }
}
