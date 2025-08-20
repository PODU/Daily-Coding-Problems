// Pancake sort: only primitive is reverse(lst,i,j). Each round reverse the window's max into place. O(n^2) time, O(1) space.
import java.util.*;

public class Solution {
    static void reverse(int[] a, int i, int j) {
        while (i < j) { int t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
    }

    static void pancakeSort(int[] a) {
        int n = a.length;
        for (int size = n; size > 1; size--) {
            int maxIdx = 0;
            for (int k = 1; k < size; k++) if (a[k] > a[maxIdx]) maxIdx = k;
            if (maxIdx != size - 1) reverse(a, maxIdx, size - 1);
        }
    }

    public static void main(String[] args) {
        int[] a = {3, 6, 1, 5, 2, 4};
        pancakeSort(a);
        System.out.println(Arrays.toString(a)); // [1, 2, 3, 4, 5, 6]
    }
}
