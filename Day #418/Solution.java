// Day 418: Two-pass greedy. Each gets >= 1; more than any lower neighbor. Like candy distribution.
// Time O(n), Space O(n).
import java.util.Arrays;
import java.util.stream.Collectors;

public class Solution {
    static int[] bonuses(int[] lines) {
        int n = lines.length;
        int[] res = new int[n];
        Arrays.fill(res, 1);
        for (int i = 1; i < n; i++)
            if (lines[i] > lines[i-1]) res[i] = res[i-1] + 1;
        for (int i = n - 2; i >= 0; i--)
            if (lines[i] > lines[i+1]) res[i] = Math.max(res[i], res[i+1] + 1);
        return res;
    }

    public static void main(String[] args) {
        int[] lines = {10, 40, 200, 1000, 60, 30};
        int[] res = bonuses(lines);
        String out = Arrays.stream(res).mapToObj(Integer::toString)
                .collect(Collectors.joining(", ", "[", "]"));
        System.out.println(out);
    }
}
