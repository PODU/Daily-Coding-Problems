// Dutch national flag: one-pass 3-way partition. Time O(n), Space O(1).
import java.util.Arrays;

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
        char[] a = {'G', 'B', 'R', 'R', 'B', 'R', 'G'};
        sortRGB(a);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < a.length; i++) {
            sb.append("'").append(a[i]).append("'");
            if (i + 1 < a.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }
}
