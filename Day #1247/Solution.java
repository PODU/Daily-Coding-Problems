// Squares of a sorted array via two-pointer merge from both ends. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int[] sortedSquares(int[] a) {
        int n = a.length;
        int[] res = new int[n];
        int l = 0, r = n - 1;
        for (int i = n - 1; i >= 0; --i) {
            int ls = a[l] * a[l], rs = a[r] * a[r];
            if (ls > rs) { res[i] = ls; ++l; }
            else { res[i] = rs; --r; }
        }
        return res;
    }

    public static void main(String[] args) {
        int[] input = {-9, -2, 0, 2, 3};
        int[] res = sortedSquares(input);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; ++i) {
            sb.append(res[i]);
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
