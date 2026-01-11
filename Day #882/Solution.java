// 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
// State = 9-char string, 0 = blank. Time/space depend on solution depth.
import java.util.*;

public class Solution {
    static final String GOAL = "123456780";

    static int manhattan(String s){
        int d = 0;
        for(int i = 0; i < 9; i++){
            if(s.charAt(i) == '0') continue;
            int v = s.charAt(i) - '1';
            d += Math.abs(i / 3 - v / 3) + Math.abs(i % 3 - v % 3);
        }
        return d;
    }

    static List<Integer> solve(String start){
        PriorityQueue<Object[]> pq = new PriorityQueue<>(Comparator.comparingInt(a -> (int) a[0]));
        Map<String,Integer> g = new HashMap<>();
        Map<String,String> parentState = new HashMap<>();
        Map<String,Integer> parentTile = new HashMap<>();
        g.put(start, 0);
        pq.add(new Object[]{manhattan(start), 0, start});
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while(!pq.isEmpty()){
            Object[] top = pq.poll();
            int gc = (int) top[1];
            String cur = (String) top[2];
            if(cur.equals(GOAL)) break;
            if(gc > g.getOrDefault(cur, Integer.MAX_VALUE)) continue;
            int z = cur.indexOf('0'), r = z / 3, c = z % 3;
            for(int k = 0; k < 4; k++){
                int nr = r + dr[k], nc = c + dc[k];
                if(nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
                int nz = nr * 3 + nc;
                char[] arr = cur.toCharArray();
                char tmp = arr[z]; arr[z] = arr[nz]; arr[nz] = tmp;
                String nxt = new String(arr);
                int ng = gc + 1;
                if(ng < g.getOrDefault(nxt, Integer.MAX_VALUE)){
                    g.put(nxt, ng);
                    parentState.put(nxt, cur);
                    parentTile.put(nxt, cur.charAt(nz) - '0');
                    pq.add(new Object[]{ng + manhattan(nxt), ng, nxt});
                }
            }
        }
        LinkedList<Integer> moves = new LinkedList<>();
        String cur = GOAL;
        while(!cur.equals(start)){
            moves.addFirst(parentTile.get(cur));
            cur = parentState.get(cur);
        }
        return moves;
    }

    public static void main(String[] args){
        String start = "123406758"; // [[1,2,3],[4,_,6],[7,5,8]]
        List<Integer> moves = solve(start);
        System.out.println("Solved in " + moves.size() + " moves: " + moves);
    }
}
