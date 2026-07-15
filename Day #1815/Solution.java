// Gray code via reflect-and-prefix formula g(i) = i XOR (i>>1).
// Time: O(n * 2^n) to format. Space: O(2^n).
import java.util.*;

public class Solution {
    static List<String> grayCode(int n) {
        List<String> res = new ArrayList<>();
        for (int i = 0; i < (1 << n); i++) {
            int g = i ^ (i >> 1);
            StringBuilder sb = new StringBuilder();
            for (int b = n - 1; b >= 0; b--) sb.append(((g >> b) & 1) == 1 ? '1' : '0');
            res.add(sb.toString());
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println(grayCode(2)); // [00, 01, 11, 10]
    }
}
