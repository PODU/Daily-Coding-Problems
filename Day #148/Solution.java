// Gray code generation via reflect formula i ^ (i>>1). Time O(2^n), Space O(2^n).
import java.util.*;

public class Solution {
    static List<String> grayCode(int n) {
        List<String> res = new ArrayList<>();
        for (int i = 0; i < (1 << n); i++) {
            int g = i ^ (i >> 1);
            StringBuilder sb = new StringBuilder();
            for (int b = n - 1; b >= 0; b--)
                sb.append((g >> b) & 1);
            res.add(sb.toString());
        }
        return res;
    }

    public static void main(String[] args) {
        int n = 2;
        List<String> codes = grayCode(n);
        StringBuilder out = new StringBuilder("[");
        for (int i = 0; i < codes.size(); i++) {
            out.append(codes.get(i));
            if (i + 1 < codes.size()) out.append(", ");
        }
        out.append("]");
        System.out.println(out.toString());
    }
}
