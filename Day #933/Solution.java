// Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
// Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
// Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.
import java.util.Arrays;

public class Solution {
    // signs[0] is the sentinel for None; signs[i] in {'+','-'} for i>=1.
    static int[] reconstruct(char[] signs) {
        int n = signs.length;
        int lo = 0, hi = n - 1;
        int[] res = new int[n];
        int idx = 0;
        for (int i = 1; i < n; i++) {
            if (signs[i] == '+') res[idx++] = lo++;
            else                 res[idx++] = hi--;
        }
        res[idx] = lo; // lo == hi
        return res;
    }

    public static void main(String[] args) {
        char[] signs = {'?', '+', '+', '-', '+'}; // [None, +, +, -, +]
        System.out.println(Arrays.toString(reconstruct(signs))); // e.g. [0, 1, 4, 2, 3]
    }
}
