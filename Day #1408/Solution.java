// Pancake sort: for each shrinking prefix find its max, flip it to the front,
// then flip it into its final position. Only uses reverse(lst,i,j).
// Time O(n^2) comparisons, O(n) flips, Space O(1).
import java.util.*;

public class Solution {
    static void reverse(int[] a, int i, int j) {
        while (i < j) { int t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
    }

    static void pancakeSort(int[] a) {
        for (int n = a.length; n > 1; n--) {
            int mi = 0;
            for (int i = 1; i < n; i++) if (a[i] > a[mi]) mi = i;
            if (mi != n - 1) {
                reverse(a, 0, mi);
                reverse(a, 0, n - 1);
            }
        }
    }

    public static void main(String[] args) {
        int[] a = {3, 1, 4, 1, 5, 9, 2, 6};
        pancakeSort(a);
        System.out.println(Arrays.toString(a));
    }
}
