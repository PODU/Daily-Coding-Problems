// Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
// Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).
import java.util.*;

public class Solution {
    static List<Long> egyptian(long a, long b) {
        List<Long> denoms = new ArrayList<>();
        while (a != 0) {
            long d = (b + a - 1) / a; // ceil(b/a)
            denoms.add(d);
            a = a * d - b;
            b = b * d;
        }
        return denoms;
    }

    public static void main(String[] args) {
        List<Long> denoms = egyptian(4, 13);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < denoms.size(); i++) {
            sb.append("1 / ").append(denoms.get(i));
            if (i + 1 < denoms.size()) sb.append(" + ");
        }
        System.out.println(sb.toString());
    }
}
