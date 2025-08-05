// Count occurrences of X in N×N table: for each row i (1..N), X appears iff i|X and X/i in [1..N].
// Time O(N), Space O(1).
public class Solution {
    static int countX(long N, long X) {
        int cnt = 0;
        for (long i = 1; i <= N; i++)
            if (X % i == 0 && X / i >= 1 && X / i <= N) cnt++;
        return cnt;
    }

    public static void main(String[] args) {
        System.out.println(countX(6, 12));
    }
}
