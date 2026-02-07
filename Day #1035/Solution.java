// Day 1035: Smallest bonuses so each employee beats any lower-output neighbor.
// Two-pass greedy: left-to-right then right-to-left taking max. Time O(n), Space O(n).
import java.util.Arrays;

public class Solution {
    static int[] bonuses(int[] a) {
        int n = a.length;
        int[] b = new int[n];
        Arrays.fill(b, 1);
        for (int i = 1; i < n; i++)
            if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
        for (int i = n - 2; i >= 0; i--)
            if (a[i] > a[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
        return b;
    }

    public static void main(String[] args) {
        int[] a = {10, 40, 200, 1000, 60, 30};
        System.out.println(Arrays.toString(bonuses(a)));
    }
}
