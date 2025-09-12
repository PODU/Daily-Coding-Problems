// Day 264: De Bruijn sequence B(k, n) over a character set.
// Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
// whose length divides n, generated via Duval-style recursion.
// Time O(k^n) (size of the output), Space O(n).

public class Solution {
    private char[] alphabet;
    private int n, k;
    private int[] a;
    private StringBuilder sequence = new StringBuilder();

    Solution(char[] alphabet, int n) {
        this.alphabet = alphabet;
        this.n = n;
        this.k = alphabet.length;
        this.a = new int[k * n];
    }

    private void db(int t, int p) {
        if (t > n) {
            if (n % p == 0)
                for (int i = 1; i <= p; i++) sequence.append(alphabet[a[i]]);
        } else {
            a[t] = a[t - p];
            db(t + 1, p);
            for (int j = a[t - p] + 1; j < k; j++) {
                a[t] = j;
                db(t + 1, t);
            }
        }
    }

    String build() {
        db(1, 1);
        return sequence.toString();
    }

    public static void main(String[] args) {
        // C = {0, 1}, k = 3
        Solution d = new Solution(new char[]{'0', '1'}, 3);
        System.out.println(d.build());
    }
}
