// De Bruijn sequence B(k_alphabet, n) via the FKM/Lyndon-word recursion.
// Concatenating Lyndon words whose length divides n. Time: O(k^n). Space: O(k^n).
import java.util.*;

public class Solution {
    static char[] C;
    static int n, k;
    static int[] a;
    static StringBuilder seq;

    static void db(int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int j = 1; j <= p; j++) seq.append(C[a[j]]);
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    }

    static String deBruijn(char[] alphabet, int order) {
        C = alphabet; n = order; k = alphabet.length;
        a = new int[k * n]; seq = new StringBuilder();
        db(1, 1);
        return seq.toString();
    }

    public static void main(String[] args) {
        System.out.println(deBruijn(new char[]{'0', '1'}, 3)); // 00010111
    }
}
