// Day 850: De Bruijn sequence via the FKM (Lyndon-word) algorithm.
// Lexicographically smallest cyclic sequence containing every length-n string once. O(k^n).
public class Solution {
    static int k, n;
    static int[] a;
    static StringBuilder seq;
    static String alphabet;

    static void db(int t, int p){
        if(t > n){
            if(n % p == 0)
                for(int j = 1; j <= p; j++) seq.append(alphabet.charAt(a[j]));
        } else {
            a[t] = a[t-p];
            db(t+1, p);
            for(int j = a[t-p] + 1; j < k; j++){ a[t] = j; db(t+1, t); }
        }
    }

    static String deBruijn(int kAlpha, int sub, String alpha){
        k = kAlpha; n = sub; alphabet = alpha;
        a = new int[k * n];
        seq = new StringBuilder();
        db(1, 1);
        return seq.toString();
    }

    public static void main(String[] args){
        // C = {0,1}, length 3 => alphabet size 2, n = 3
        System.out.println(deBruijn(2, 3, "01")); // 00010111
    }
}
