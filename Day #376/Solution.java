// Closest coin by Manhattan distance. Linear scan.
// Time: O(n), Space: O(1).
import java.util.*;

public class Solution {
    static int[] closestCoin(int[] me, int[][] coins){
        int[] best = null;
        long bestD = Long.MAX_VALUE;
        for(int[] c : coins){
            long d = Math.abs(c[0]-me[0]) + Math.abs(c[1]-me[1]);
            if(d < bestD){ bestD = d; best = c; }
        }
        return best;
    }

    public static void main(String[] args){
        int[] me = {0,2};
        int[][] coins = {{0,4},{1,0},{2,0},{3,2}};
        int[] b = closestCoin(me, coins);
        System.out.println("(" + b[0] + ", " + b[1] + ")");
    }
}
