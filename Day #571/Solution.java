// Tower of Hanoi: classic recursion. Move n disks from rod `from` to `to` using `via`.
// Prints 2^n - 1 moves. O(2^n) time, O(n) recursion depth.
public class Solution {
    static void hanoi(int n, int from, int to, int via) {
        if (n == 0) return;
        hanoi(n - 1, from, via, to);
        System.out.println("Move " + from + " to " + to);
        hanoi(n - 1, via, to, from);
    }

    public static void main(String[] args) {
        hanoi(3, 1, 3, 2);
    }
}
