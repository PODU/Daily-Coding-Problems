// Day 1624: Steps of Kaprekar's routine to reach 6174.
// Iterate sort-desc minus sort-asc until 6174. Time O(1) (bounded ~7 iters).
import java.util.*;

public class Solution {
    static int kaprekarSteps(int n) {
        int steps = 0;
        while (n != 6174) {
            char[] c = String.format("%04d", n).toCharArray();
            Arrays.sort(c);
            String asc = new String(c);
            String desc = new StringBuilder(asc).reverse().toString();
            n = Integer.parseInt(desc) - Integer.parseInt(asc);
            steps++;
            if (n == 0) break;
        }
        return steps;
    }

    public static void main(String[] args) {
        System.out.println(kaprekarSteps(1234));
    }
}
