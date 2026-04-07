// Reconstruct a permutation of [0..N] consistent with up/down signs.
// Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
// Any consistent array is valid; README shows one such answer.
import java.util.*;

public class Solution {
    // signs[0] is the sentinel (None); signs[i] in {'+','-'} for i>=1.
    static int[] reconstruct(char[] signs) {
        int n = signs.length, N = n - 1;
        int[] res = new int[n];
        int lo = 0, hi = N, idx = 0;
        for (int i = 1; i < n; i++) {
            res[idx++] = (signs[i] == '+') ? lo++ : hi--;
        }
        res[idx] = lo; // lo == hi for the final element
        return res;
    }

    static boolean consistent(char[] s, int[] a) {
        for (int i = 1; i < s.length; i++) {
            if (s[i] == '+' && !(a[i] > a[i-1])) return false;
            if (s[i] == '-' && !(a[i] < a[i-1])) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        char[] signs = {'#', '+', '+', '-', '+'}; // '#' stands in for None
        int[] a = reconstruct(signs);
        System.out.println(Arrays.toString(a) + "  consistent=" + consistent(signs, a));
        // -> [0, 1, 4, 2, 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
    }
}
