// Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
// '+' takes the current low, '-' takes the current high; produces a valid order.
import java.util.*;

public class Solution {
    static int[] reconstruct(char[] signs){ // '+'/'-' constraints (None dropped)
        int L = signs.length + 1, N = L - 1;
        int[] res = new int[L];
        int low = 0, high = N;
        for(int j = 0; j < signs.length; j++){
            if(signs[j] == '+') res[j] = low++;
            else res[j] = high--;
        }
        res[L-1] = low;
        return res;
    }
    public static void main(String[] args){
        // input [None, +, +, -, +] -> constraints (+,+,-,+)
        char[] signs = {'+','+','-','+'};
        int[] r = reconstruct(signs);
        System.out.println(Arrays.toString(r)); // a valid reconstruction, e.g. [0, 1, 4, 2, 3]
    }
}
