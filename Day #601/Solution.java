// Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
// Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.
import java.util.*;

public class Solution {
    // signs[0] is the placeholder (None); signs[k] is '+' if a[k] > a[k-1], else '-'.
    static int[] reconstruct(char[] signs) {
        int n = signs.length;       // numbers 0..n-1
        int low = 0, high = n - 1, idx = 0;
        int[] res = new int[n];
        for (int k = 1; k < n; k++) {
            if (signs[k] == '+') res[idx++] = low++;
            else res[idx++] = high--;
        }
        res[idx] = low;
        return res;
    }

    public static void main(String[] args) {
        char[] signs = {' ', '+', '+', '-', '+'}; // [None, +, +, -, +]
        int[] a = reconstruct(signs);
        System.out.println(Arrays.toString(a));
    }
}
