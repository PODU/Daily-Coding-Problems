// Next greater permutation in-place. Standard next_permutation.
// Time O(n), Space O(1).
import java.util.*;

public class Solution {
    static void nextPermutation(int[] a) {
        int n = a.length, i = n - 2;
        while (i >= 0 && a[i] >= a[i + 1]) i--;
        if (i >= 0) {
            int j = n - 1;
            while (a[j] <= a[i]) j--;
            int t = a[i]; a[i] = a[j]; a[j] = t;
        }
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            int t = a[l]; a[l] = a[r]; a[r] = t;
        }
    }

    static String fmt(int[] a) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < a.length; i++) {
            if (i > 0) sb.append(", ");
            sb.append(a[i]);
        }
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        int[][] cases = {{1,2,3},{1,3,2},{3,2,1}};
        for (int[] c : cases) {
            nextPermutation(c);
            System.out.println(fmt(c));
        }
    }
}
