// De Bruijn sequence via FKM (Lyndon-word/necklace) algorithm: emit Lyndon words whose
// length divides n, in order, giving lexicographically smallest sequence. Time O(k^n).
public class Solution {
    static int k, n;
    static int[] a;
    static StringBuilder seq;

    static void db(int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int j = 1; j <= p; j++) seq.append((char) ('0' + a[j]));
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    }

    static String deBruijn(int kk, int nn) {
        k = kk; n = nn;
        a = new int[k * n + 1];
        seq = new StringBuilder();
        db(1, 1);
        return seq.toString();
    }

    public static void main(String[] args) {
        System.out.println(deBruijn(2, 3)); // 00010111
    }
}
