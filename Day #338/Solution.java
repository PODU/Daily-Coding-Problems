// Next higher integer with same number of set bits (snoob bit-twiddle).
// O(1) time, O(1) space.
public class Solution {
    static int nextSameBits(int n) {
        int smallest = n & (-n);
        int ripple = n + smallest;
        int ones = n ^ ripple;
        ones = (ones >>> 2) / smallest;
        return ripple | ones;
    }

    public static void main(String[] args) {
        System.out.println(nextSameBits(6)); // 9
    }
}
