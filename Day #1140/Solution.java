// De Bruijn B(k,n) via FKM (Lyndon word) algorithm -> lexicographically smallest. O(k^n).
public class Solution {
    static int K, N;
    static int[] a;
    static StringBuilder seq;

    static void db(int t, int p) {
        if (t > N) {
            if (N % p == 0)
                for (int i = 1; i <= p; i++) seq.append((char) ('0' + a[i]));
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < K; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    }

    static String deBruijn(int k, int n) {
        K = k; N = n;
        a = new int[k * n + 1];
        seq = new StringBuilder();
        db(1, 1);
        return seq.toString();
    }

    public static void main(String[] args) {
        // C = {0,1} -> alphabet size 2, substring length 3
        System.out.println(deBruijn(2, 3));
    }
}
