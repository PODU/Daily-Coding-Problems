// Product of array except self without division.
// Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
import java.util.StringJoiner;

public class Solution {
    static long[] productExceptSelf(long[] a) {
        int n = a.length;
        long[] res = new long[n];
        long prefix = 1;
        for (int i = 0; i < n; i++) { res[i] = prefix; prefix *= a[i]; }
        long suffix = 1;
        for (int i = n - 1; i >= 0; i--) { res[i] *= suffix; suffix *= a[i]; }
        return res;
    }

    static String fmt(long[] v) {
        StringJoiner sj = new StringJoiner(", ", "[", "]");
        for (long x : v) sj.add(Long.toString(x));
        return sj.toString();
    }

    public static void main(String[] args) {
        System.out.println(fmt(productExceptSelf(new long[]{1, 2, 3, 4, 5})));
        System.out.println(fmt(productExceptSelf(new long[]{3, 2, 1})));
    }
}
