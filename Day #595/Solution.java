// Kaprekar's routine: repeatedly subtract ascending-digit number from
// descending-digit number (4-digit, zero-padded) until reaching 6174.
// Bounded to <=7 steps. Time O(1), Space O(1).
import java.util.Arrays;

public class Solution {
    public static void main(String[] args) {
        int n = 1234;
        int steps = 0;
        while (n != 6174) {
            String s = String.format("%04d", n);
            char[] a = s.toCharArray();
            Arrays.sort(a);
            String asc = new String(a);
            StringBuilder sb = new StringBuilder(asc);
            String desc = sb.reverse().toString();
            int hi = Integer.parseInt(desc);
            int lo = Integer.parseInt(asc);
            n = hi - lo;
            steps++;
            System.out.printf("%04d - %04d = %04d%n", hi, lo, n);
        }
        System.out.printf("Steps: %d%n", steps);
    }
}
