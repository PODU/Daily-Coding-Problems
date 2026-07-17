// Egg drop: min trials t s.t. floors(t,eggs)=sum_{i=1..eggs} C(t,i) >= k.
// O(eggs * answer). For N=1,k=5 -> 5.
public class Solution {
    static long floorsCovered(long t, long eggs, long cap){
        long total = 0, c = 1;
        for(long i = 1; i <= eggs; i++){
            c = c * (t - i + 1) / i;   // C(t,i) incrementally
            total += c;
            if(total >= cap) return cap;
        }
        return total;
    }
    static long minDrops(long eggs, long k){
        long t = 0;
        while(floorsCovered(t, eggs, k) < k) t++;
        return t;
    }
    public static void main(String[] args){
        System.out.println(minDrops(1, 5)); // 5
    }
}
