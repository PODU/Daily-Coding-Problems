// Day 1028: Kaprekar's routine. Repeatedly subtract ascending- from descending-
// digit arrangement until 6174; count steps. Time O(steps), Space O(1).
import java.util.Arrays;

public class Solution {
    static int kaprekarSteps(int n) {
        int steps = 0;
        while (n != 6174) {
            char[] c = String.format("%04d", n).toCharArray();
            Arrays.sort(c);
            int asc = Integer.parseInt(new String(c));
            StringBuilder sb = new StringBuilder(new String(c)).reverse();
            int desc = Integer.parseInt(sb.toString());
            n = desc - asc;
            steps++;
        }
        return steps;
    }

    public static void main(String[] args) {
        System.out.println(kaprekarSteps(1234));
    }
}
