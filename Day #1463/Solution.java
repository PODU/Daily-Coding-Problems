// Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
// Time: O(n^2), Space: O(n).
import java.util.*;

public class Solution {
    public static boolean hasTriplet(int[] input) {
        int n = input.length;
        long[] arr = new long[n];
        for (int i = 0; i < n; i++) arr[i] = (long) input[i] * input[i];
        Arrays.sort(arr);
        for (int c = n - 1; c >= 2; c--) {
            int l = 0, r = c - 1;
            while (l < r) {
                long s = arr[l] + arr[r];
                if (s == arr[c]) return true;
                else if (s < arr[c]) l++;
                else r--;
            }
        }
        return false;
    }

    public static void main(String[] args) {
        System.out.println(hasTriplet(new int[]{3, 1, 4, 6, 5}) ? "True" : "False");
        System.out.println(hasTriplet(new int[]{10, 4, 6, 12, 5}) ? "True" : "False");
    }
}
