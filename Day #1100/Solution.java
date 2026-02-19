// Day 1100: Search sorted array in O(log N) using only addition/comparison
// (no *, /, or bit-shift). Binary lifting with powers of two built by doubling.
// Time: O(log N). Space: O(log N).
import java.util.*;

public class Solution {
    static boolean contains(int[] a, int x){
        int n = a.length;
        if (n == 0) return false;
        ArrayList<Integer> pows = new ArrayList<>();
        for (int p = 1; p <= n; p += p) pows.add(p);
        int pos = -1;
        for (int i = pows.size()-1; i >= 0; i--){
            int p = pows.get(i);
            if (pos + p < n && a[pos + p] <= x) pos += p;
        }
        return pos >= 0 && a[pos] == x;
    }

    public static void main(String[] args){
        int[] a = {1,3,5,7,9,11};
        System.out.println(contains(a, 7)); // true
        System.out.println(contains(a, 8)); // false
    }
}
