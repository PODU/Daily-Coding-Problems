// Day 128: Tower of Hanoi - print all moves.
// Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.
public class Solution {
    static void hanoi(int n, int from, int via, int to) {
        if (n == 0) return;
        hanoi(n - 1, from, to, via);
        System.out.println("Move " + from + " to " + to);
        hanoi(n - 1, via, from, to);
    }

    public static void main(String[] args) {
        hanoi(3, 1, 2, 3);
    }
}
