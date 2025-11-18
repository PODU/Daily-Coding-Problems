// Pancake sort using only reverse(lst,i,j): flip current max to front, then to its place.
// Time: O(n^2) comparisons, O(n) flips, Space: O(1).
import java.util.*;

public class Solution {
    static void reverse(int[] lst, int i, int j) {
        while (i < j) { int t = lst[i]; lst[i] = lst[j]; lst[j] = t; i++; j--; }
    }

    static void pancakeSort(int[] lst) {
        for (int size = lst.length; size > 1; size--) {
            int maxIdx = 0;
            for (int k = 1; k < size; k++) if (lst[k] > lst[maxIdx]) maxIdx = k;
            if (maxIdx != size - 1) {
                if (maxIdx != 0) reverse(lst, 0, maxIdx);
                reverse(lst, 0, size - 1);
            }
        }
    }

    public static void main(String[] args) {
        int[] lst = {3, 1, 4, 1, 5, 9, 2, 6};
        pancakeSort(lst);
        System.out.println(Arrays.toString(lst));
    }
}
