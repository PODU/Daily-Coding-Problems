// Day 85: Select x if b==1 else y using only bit ops. mask = -b (all 1s or all 0s).
// Time O(1), Space O(1).
public class Solution {
    static int select(int x, int y, int b) {
        int mask = -b;                  // b=1 -> 0xFFFFFFFF, b=0 -> 0x00000000
        return (x & mask) | (y & ~mask);
    }

    public static void main(String[] args) {
        System.out.println(select(5, 10, 1)); // 5
        System.out.println(select(5, 10, 0)); // 10
    }
}
