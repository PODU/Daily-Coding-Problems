// Reflected binary Gray code: g(i) = i ^ (i>>1). Time O(2^n), Space O(2^n).
import java.util.*;

public class Solution {
    static List<String> grayCode(int n) {
        List<String> res = new ArrayList<>();
        for (int i = 0; i < (1 << n); i++) {
            int g = i ^ (i >> 1);
            StringBuilder bits = new StringBuilder();
            for (int b = n - 1; b >= 0; b--) bits.append(((g >> b) & 1) == 1 ? '1' : '0');
            res.add(bits.toString());
        }
        return res;
    }

    public static void main(String[] args) {
        System.out.println("[" + String.join(", ", grayCode(2)) + "]");
    }
}
