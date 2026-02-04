// Pancake sort using only reverse(lst,i,j): for each end, bring max of prefix to front then flip to its spot.
// O(n^2) comparisons, O(n) reversals, in place. Space O(1).
import java.util.*;

public class Solution {
    static void reverse(int[] lst, int i, int j) {
        while (i < j) { int t = lst[i]; lst[i] = lst[j]; lst[j] = t; i++; j--; }
    }

    static void pancakeSort(int[] lst) {
        int n = lst.length;
        for (int end = n - 1; end > 0; end--) {
            int mi = 0;
            for (int k = 1; k <= end; k++) if (lst[k] > lst[mi]) mi = k;
            if (mi == end) continue;
            if (mi != 0) reverse(lst, 0, mi); // bring max to front
            reverse(lst, 0, end);             // move max to its final position
        }
    }

    public static void main(String[] args) {
        int[] arr = {3, 1, 4, 1, 5, 9, 2, 6};
        pancakeSort(arr);
        System.out.println(Arrays.toString(arr));
    }
}
