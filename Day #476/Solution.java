// Find duplicate in array of n+1 ints from 1..n using a visited/seen set.
// Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
import java.util.*;

public class Solution {
    static int findDuplicate(int[] a) {
        boolean[] seen = new boolean[a.length + 1];
        for (int x : a) {
            if (seen[x]) return x;
            seen[x] = true;
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] a = {1, 3, 4, 2, 2};
        System.out.println(findDuplicate(a));
    }
}
