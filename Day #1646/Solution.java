// Rotate a list LEFT by k via three in-place reversals: reverse[0,k-1], reverse[k,n-1], reverse[0,n-1].
// Time O(n), Space O(1).
public class Solution {
    static void reverseRange(int[] a, int i, int j) {
        while (i < j) { int t = a[i]; a[i] = a[j]; a[j] = t; i++; j--; }
    }

    static void rotateLeft(int[] a, int k) {
        int n = a.length;
        if (n == 0) return;
        k %= n;
        reverseRange(a, 0, k - 1);
        reverseRange(a, k, n - 1);
        reverseRange(a, 0, n - 1);
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6};
        rotateLeft(a, 2);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < a.length; i++) {
            if (i > 0) sb.append(", ");
            sb.append(a[i]);
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
