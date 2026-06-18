// Selection sort using only reverse(lst,i,j). For each i, find min in [i..n-1],
// reverse [i..m] to bring it to front. Time O(n^2), Space O(1).
public class Solution {
    static void reverseRange(int[] lst, int i, int j) {
        while (i < j) { int t = lst[i]; lst[i] = lst[j]; lst[j] = t; i++; j--; }
    }

    static void sortWithReverse(int[] lst) {
        int n = lst.length;
        for (int i = 0; i < n; i++) {
            int m = i;
            for (int k = i + 1; k < n; k++) if (lst[k] < lst[m]) m = k;
            reverseRange(lst, i, m);
        }
    }

    public static void main(String[] args) {
        int[] lst = {3, 1, 2, 5, 4};
        sortWithReverse(lst);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < lst.length; i++) {
            sb.append(lst[i]);
            if (i + 1 < lst.length) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
