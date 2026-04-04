// Day 1301: Fewest jumps from 0 to N where the ith jump moves exactly i (left/right).
// Find smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even (flipping a jump changes sum by 2*val).
public class Solution {
    static int minJumps(long N) {
        N = Math.abs(N);
        long k = 0, sum = 0;
        while (sum < N || (sum - N) % 2 != 0) {
            k++;
            sum += k;
        }
        return (int) k;
    }

    public static void main(String[] args) {
        System.out.println(minJumps(3)); // 2
        System.out.println(minJumps(2)); // 3
    }
}
