// Day 448: Dutch National Flag sort of R/G/B. O(n) time, O(1) space, in-place
// with three pointers (low=R boundary, high=B boundary, mid=scanner).
import java.util.*;

public class Solution {
    static void sortRGB(char[] a) {
        int low = 0, mid = 0, high = a.length - 1;
        while (mid <= high) {
            if (a[mid] == 'R') { char t = a[low]; a[low] = a[mid]; a[mid] = t; low++; mid++; }
            else if (a[mid] == 'G') mid++;
            else { char t = a[mid]; a[mid] = a[high]; a[high] = t; high--; }
        }
    }

    public static void main(String[] args) {
        char[] a = {'G','B','R','R','B','R','G'};
        sortRGB(a);
        List<String> out = new ArrayList<>();
        for (char c : a) out.add("'" + c + "'");
        System.out.println("[" + String.join(", ", out) + "]");
        // ['R', 'R', 'R', 'G', 'G', 'B', 'B']
    }
}
