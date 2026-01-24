// Day 948: Tower of Hanoi - print all moves to move n disks from rod 1 to rod 3.
// Classic recursion. Time O(2^n) moves, Space O(n) recursion depth.
public class Solution {
    static void hanoi(int n, int from, int to, int aux) {
        if (n == 0) return;
        hanoi(n - 1, from, aux, to);
        System.out.println("Move " + from + " to " + to);
        hanoi(n - 1, aux, to, from);
    }

    public static void main(String[] args) {
        int n = 3;
        hanoi(n, 1, 3, 2);
    }
}
