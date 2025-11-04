// Domino + Tromino tiling of 2xN: f(n)=2*f(n-1)+f(n-3), f(0)=1,f(1)=1,f(2)=2.
// O(n) time, O(1) space.
public class Solution {
    static long tilings(int n) {
        if (n == 0) return 1;
        if (n == 1) return 1;
        if (n == 2) return 2;
        long a = 1, b = 1, c = 2, cur = c;
        for (int i = 3; i <= n; i++) {
            cur = 2 * c + a;
            a = b; b = c; c = cur;
        }
        return cur;
    }

    public static void main(String[] args) {
        int N = 4;
        System.out.println(tilings(N)); // 11
    }
}
