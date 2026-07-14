// Tower of Hanoi: recursive divide-and-conquer.
// Time: O(2^n) moves (optimal/minimal). Space: O(n) recursion depth.
public class Solution {
    static void hanoi(int n, int from, int to, int via) {
        if (n == 0) return;
        hanoi(n - 1, from, via, to);
        System.out.println("Move " + from + " to " + to);
        hanoi(n - 1, via, to, from);
    }

    public static void main(String[] args) {
        int n = 3;
        hanoi(n, 1, 3, 2);
    }
}
