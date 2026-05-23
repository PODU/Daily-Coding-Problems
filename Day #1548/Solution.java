// Candy problem: init bonuses to 1, left-to-right then right-to-left passes enforce strict ordering.
// Time O(n), Space O(n).
import java.util.StringJoiner;

public class Solution {
    static int[] bonuses(int[] a) {
        int n = a.length;
        int[] b = new int[n];
        for (int i = 0; i < n; i++) b[i] = 1;
        for (int i = 1; i < n; i++)
            if (a[i] > a[i - 1]) b[i] = b[i - 1] + 1;
        for (int i = n - 2; i >= 0; i--)
            if (a[i] > a[i + 1]) b[i] = Math.max(b[i], b[i + 1] + 1);
        return b;
    }

    public static void main(String[] args) {
        int[] a = {10, 40, 200, 1000, 60, 30};
        int[] b = bonuses(a);
        StringJoiner sj = new StringJoiner(", ", "[", "]");
        for (int x : b) sj.add(Integer.toString(x));
        System.out.println(sj.toString());
    }
}
