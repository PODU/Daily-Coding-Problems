// Brick wall: prefix-sum edge positions per row, max edge frequency => fewest cuts = rows - maxEdges. O(total bricks) time/space.
import java.util.*;
public class Solution {
    public static void main(String[] args){
        int[][] wall={{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
        Map<Long,Integer> freq=new HashMap<>();
        int best=0;
        for(int[] row:wall){
            long sum=0;
            for(int i=0;i+1<row.length;i++){ sum+=row[i]; int c=freq.merge(sum,1,Integer::sum); best=Math.max(best,c); }
        }
        System.out.println(wall.length-best);
    }
}
