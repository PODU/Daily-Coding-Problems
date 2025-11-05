// Count occurrences of X in an N x N multiplication table.
// For each row i (1..N), X appears iff i divides X and X/i is in [1,N]. O(N) time, O(1) space.
public class Solution {
    static int countX(int N, long X) {
        int cnt = 0;
        for (int i = 1; i <= N; i++)
            if (X % i == 0) {
                long q = X / i;
                if (q >= 1 && q <= N) cnt++;
            }
        return cnt;
    }

    public static void main(String[] args) {
        System.out.println(countX(6, 12)); // 4
    }
}
