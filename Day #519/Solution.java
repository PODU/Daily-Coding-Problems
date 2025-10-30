// Branchless select: mask = -b (all 1s if b=1, all 0s if b=0); pick x or y. O(1).
public class Solution {
    static int select(int x, int y, int b) {
        int mask = -b;
        return (x & mask) | (y & ~mask);
    }

    public static void main(String[] args) {
        System.out.println(select(42, 17, 1)); // 42
        System.out.println(select(42, 17, 0)); // 17
    }
}
