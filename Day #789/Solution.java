// Next lexicographic permutation in place (classic next_permutation). O(n) time, O(1) extra space.
import java.util.Arrays;

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

    static void print(int[] a) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < a.length; i++) {
            sb.append(a[i]);
            if (i + 1 < a.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3}, b = {1, 3, 2}, c = {3, 2, 1};
        nextPermutation(a); print(a);
        nextPermutation(b); print(b);
        nextPermutation(c); print(c);
    }
}
