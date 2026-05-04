// Day 1471: O(log N) search in a sorted array with no *, /, or bit-shift.
// Build powers of two by doubling (addition); jump-based binary search.
// Time O(log N), Space O(log N).
import java.util.*;

public class Solution {
    static boolean search(int[] a, int x) {
        int n = a.length;
        if (n == 0) return false;
        List<Integer> powers = new ArrayList<>();
        for (int p = 1; p <= n; p = p + p) powers.add(p);
        int pos = -1;
        for (int i = powers.size() - 1; i >= 0; --i) {
            int nxt = pos + powers.get(i);
            if (nxt < n && a[nxt] <= x) pos = nxt;
        }
        return pos >= 0 && a[pos] == x;
    }

    public static void main(String[] args) {
        int[] arr = {1, 3, 5, 7, 9, 11};
        System.out.println(search(arr, 7) ? "True" : "False");
        System.out.println(search(arr, 8) ? "True" : "False");
    }
}
