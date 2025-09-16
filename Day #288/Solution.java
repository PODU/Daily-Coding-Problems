// Kaprekar's routine: repeatedly subtract ascending-digit from descending-digit
// (4-digit, zero-padded) until 6174; count steps. Time: O(7) iters, Space: O(1).
import java.util.*;

public class Solution {
    static int kaprekar(int n) {
        int steps = 0;
        while (n != 6174) {
            char[] d = String.format("%04d", n).toCharArray();
            Arrays.sort(d);
            String asc = new String(d);
            StringBuilder sb = new StringBuilder(asc).reverse();
            String desc = sb.toString();
            n = Integer.parseInt(desc) - Integer.parseInt(asc);
            steps++;
        }
        return steps;
    }

    public static void main(String[] args) {
        System.out.println(kaprekar(1234));
    }
}
