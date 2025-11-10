// Min jumps: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even. Time O(sqrt(N)), Space O(1).
public class Solution {
    static int minJumps(long N) {
        long n = Math.abs(N);
        long k = 0, s = 0;
        while (s < n || (s - n) % 2 != 0) {
            k++;
            s += k;
        }
        return (int) k;
    }

    public static void main(String[] args) {
        System.out.println("Minimum jumps to reach 10: " + minJumps(10));
    }
}
