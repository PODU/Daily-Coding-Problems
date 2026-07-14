// Longest strictly increasing subsequence via patience sorting + binary search.
// Time: O(n log n). Space: O(n).
import java.util.ArrayList;
import java.util.Collections;

public class Solution {
    static int lis(int[] a) {
        ArrayList<Integer> tails = new ArrayList<>();
        for (int x : a) {
            int i = Collections.binarySearch(tails, x);
            if (i < 0) i = -(i + 1);
            if (i == tails.size()) tails.add(x);
            else tails.set(i, x);
        }
        return tails.size();
    }

    public static void main(String[] args) {
        int[] a = {0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15};
        System.out.println(lis(a)); // 6
    }
}
