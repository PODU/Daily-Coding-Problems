// Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
// Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
import java.util.*;

public class Solution {
    static int numBoats(int[] w, int k) {
        Arrays.sort(w);
        int l = 0, h = w.length - 1, boats = 0;
        while (l <= h) {
            if (w[l] + w[h] <= k) l++;
            h--;
            boats++;
        }
        return boats;
    }

    public static void main(String[] args) {
        System.out.println(numBoats(new int[]{100, 200, 150, 80}, 200));
    }
}
