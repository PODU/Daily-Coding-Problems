// Branchless select: mask = -b (all-ones if b=1 else 0); result = (x & mask) | (y & ~mask). O(1) time/space.
public class Solution {
    static int select(int b, int x, int y) {
        int mask = -b; // 0xFFFFFFFF if b=1, 0 if b=0
        return (x & mask) | (y & ~mask);
    }

    public static void main(String[] args) {
        System.out.println(select(1, 42, 99));
        System.out.println(select(0, 42, 99));
    }
}
