// Day 944: Next greater permutation of an integer's digits (in-place on digit array).
// Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).
public class Solution {
    // Returns -1 if no greater permutation exists.
    static long nextPermutation(long num) {
        char[] d = Long.toString(num).toCharArray();
        int n = d.length;
        int i = n - 2;
        while (i >= 0 && d[i] >= d[i + 1]) i--;
        if (i < 0) return -1;
        int j = n - 1;
        while (d[j] <= d[i]) j--;
        char t = d[i]; d[i] = d[j]; d[j] = t;
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            char tmp = d[l]; d[l] = d[r]; d[r] = tmp;
        }
        return Long.parseLong(new String(d));
    }

    public static void main(String[] args) {
        System.out.println(nextPermutation(48975)); // 49578
    }
}
