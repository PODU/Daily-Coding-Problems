// Day 1453: Apply a permutation P (P[i] = destination of element i) to an array
// in place by following cycles. Time O(n), Space O(1) extra (P is consumed).
import java.util.*;

public class Solution {
    static void applyPermutation(String[] arr, int[] P) {
        for (int i = 0; i < arr.length; i++) {
            while (P[i] != i) {
                int pi = P[i];
                String ts = arr[i]; arr[i] = arr[pi]; arr[pi] = ts;
                int tp = P[i]; P[i] = P[pi]; P[pi] = tp;
            }
        }
    }

    public static void main(String[] args) {
        String[] arr = {"a", "b", "c"};
        int[] P = {2, 1, 0};
        applyPermutation(arr, P);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < arr.length; i++)
            sb.append("\"").append(arr[i]).append("\"").append(i + 1 < arr.length ? ", " : "");
        sb.append("]");
        System.out.println(sb); // ["c", "b", "a"]
    }
}
