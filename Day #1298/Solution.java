// Day 1298: Count Kaprekar steps to reach 6174.
// Repeatedly sort digits desc - asc until 6174. Converges in <=7 steps. O(steps) time.
import java.util.*;

public class Solution {
    static int kaprekarSteps(int n) {
        int steps = 0;
        while (n != 6174) {
            char[] d = String.format("%04d", n).toCharArray();
            Arrays.sort(d);
            int asc = Integer.parseInt(new String(d));
            StringBuilder rev = new StringBuilder(new String(d)).reverse();
            int desc = Integer.parseInt(rev.toString());
            n = desc - asc;
            steps++;
            if (n == 0) break; // all digits equal
        }
        return steps;
    }

    public static void main(String[] args) {
        System.out.println(kaprekarSteps(1234)); // 3
    }
}
