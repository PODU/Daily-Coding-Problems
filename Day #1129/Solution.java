// Simplified Elo rating system. Expected score Ea=1/(1+10^((Rb-Ra)/400)),
// update Ra'=Ra+K*(Sa-Ea), K=32. recordGame transfers points winner<-loser. O(1) per game.
import java.util.*;

public class Solution {
    static final double K = 32.0;
    static Map<String,Double> rating = new HashMap<>();

    static void addPlayer(String name){ rating.putIfAbsent(name, 1200.0); }
    static double expected(double ra,double rb){ return 1.0/(1.0+Math.pow(10.0,(rb-ra)/400.0)); }
    static void recordGame(String winner,String loser){
        addPlayer(winner); addPlayer(loser);
        double ra=rating.get(winner), rb=rating.get(loser);
        double ea=expected(ra,rb), eb=expected(rb,ra);
        rating.put(winner, ra+K*(1.0-ea));
        rating.put(loser,  rb+K*(0.0-eb));
    }
    static int get(String name){ return (int)Math.round(rating.get(name)); }

    public static void main(String[] args){
        addPlayer("A"); rating.put("A",1200.0);
        addPlayer("B"); rating.put("B",2000.0);
        System.out.println("Before: A="+get("A")+", B="+get("B"));
        recordGame("A","B"); // lower-rated A beats higher-rated B
        System.out.println("After A(1200) beats B(2000): A="+get("A")+", B="+get("B"));
    }
}
