// Next permutation of digits: find pivot, swap with next-larger suffix digit,
// reverse suffix. Time: O(d) digits, Space: O(d).
public class Solution {
    static long nextPermutation(long num) {
        char[] d = Long.toString(num).toCharArray();
        int n = d.length;
        int i = n - 2;
        while (i >= 0 && d[i] >= d[i + 1]) i--;
        if (i < 0) return num; // already largest permutation
        int j = n - 1;
        while (d[j] <= d[i]) j--;
        char tmp = d[i]; d[i] = d[j]; d[j] = tmp;
        for (int l = i + 1, r = n - 1; l < r; l++, r--) {
            char t = d[l]; d[l] = d[r]; d[r] = t;
        }
        return Long.parseLong(new String(d));
    }

    public static void main(String[] args) {
        System.out.println(nextPermutation(48975L));
    }
}
