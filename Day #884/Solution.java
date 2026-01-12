// rand5 from rand7 via rejection sampling: keep rand7 values in 1..5. Expected O(1) calls (7/5).
import java.util.Random;

public class Solution {
    static Random rng = new Random(12345);

    static int rand7(){ return rng.nextInt(7) + 1; }

    static int rand5(){
        int x;
        do { x = rand7(); } while(x > 5);
        return x;
    }

    public static void main(String[] args){
        int[] counts = new int[6];
        int trials = 100000;
        for(int i = 0; i < trials; i++) counts[rand5()]++;
        System.out.println("Distribution over " + trials + " samples:");
        for(int v = 1; v <= 5; v++) System.out.println(v + ": " + counts[v]);
    }
}
