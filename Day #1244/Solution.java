// Boats: sort, greedily pair lightest+heaviest (two-pointer). Time O(n log n).
import java.util.Arrays;

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
        System.out.println(numBoats(new int[]{100, 200, 150, 80}, 200));
    }
}
