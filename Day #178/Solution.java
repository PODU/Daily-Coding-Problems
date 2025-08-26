// Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
// Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
import java.util.*;

public class Solution {
    static Random rng;
    static int trial(int second){
        int rolls=0, prev=0;
        while(true){ int c=rng.nextInt(6)+1; rolls++; if(prev==5 && c==second) return rolls; prev=c; }
    }
    public static void main(String[] args){
        rng = new Random(12345);
        final int T=100000;
        long s1=0, s2=0;
        for(int i=0;i<T;i++) s1+=trial(6);
        for(int i=0;i<T;i++) s2+=trial(5);
        double e1=(double)s1/T, e2=(double)s2/T;
        System.out.printf("Game 1 (five then six) expected rolls: %.2f%n", e1);
        System.out.printf("Game 2 (five then five) expected rolls: %.2f%n", e2);
        System.out.println(e1<e2
            ? "Alice should play Game 1 (five then six), it has lower expected cost."
            : "Alice should play Game 2 (five then five), it has lower expected cost.");
    }
}
