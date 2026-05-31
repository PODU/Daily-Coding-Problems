// 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
// Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
import java.util.*;

public class Solution {
    static int manhattan(int[] b){
        int d=0;
        for(int i=0;i<9;i++){
            int v=b[i];
            if(v==0) continue;
            int goalIdx=v-1;
            d += Math.abs(i/3-goalIdx/3)+Math.abs(i%3-goalIdx%3);
        }
        return d;
    }
    static String key(int[] b){
        StringBuilder sb=new StringBuilder();
        for(int x:b) sb.append(x);
        return sb.toString();
    }
    public static void main(String[] args){
        int[] start={1,2,3,4,5,6,0,7,8};
        int[] goal ={1,2,3,4,5,6,7,8,0};

        Map<String,Integer> g=new HashMap<>();
        Map<String,String> parent=new HashMap<>();
        Map<String,int[]> board=new HashMap<>();
        Map<String,Integer> movedTile=new HashMap<>();
        PriorityQueue<int[]> pq=new PriorityQueue<>((a,bb)->Integer.compare(a[0],bb[0]));
        // pq holds [f, g, hashIndex] — but we need board; store via map keyed by string. Use array carrying g and key index.
        // Simpler: store f,g and the board in a wrapper.
        PriorityQueue<Node> open=new PriorityQueue<>((a,bb)->Integer.compare(a.f,bb.f));

        String sk=key(start);
        g.put(sk,0);
        board.put(sk,start);
        parent.put(sk,sk);
        open.add(new Node(manhattan(start),0,start));

        int[] dr={-1,1,0,0}, dc={0,0,-1,1};
        boolean found=false;
        String gk=key(goal);
        while(!open.isEmpty()){
            Node n=open.poll();
            String ck=key(n.b);
            if(ck.equals(gk)){found=true;break;}
            if(n.g>g.getOrDefault(ck,Integer.MAX_VALUE)) continue;
            int z=0; while(n.b[z]!=0) z++;
            int zr=z/3, zc=z%3;
            for(int k=0;k<4;k++){
                int nr=zr+dr[k], nc=zc+dc[k];
                if(nr<0||nr>2||nc<0||nc>2) continue;
                int nz=nr*3+nc;
                int[] nb=n.b.clone();
                int tile=nb[nz];
                nb[z]=tile; nb[nz]=0;
                int ng=n.g+1;
                String nk=key(nb);
                if(ng<g.getOrDefault(nk,Integer.MAX_VALUE)){
                    g.put(nk,ng);
                    parent.put(nk,ck);
                    board.put(nk,nb);
                    movedTile.put(nk,tile);
                    open.add(new Node(ng+manhattan(nb),ng,nb));
                }
            }
        }

        List<Integer> tiles=new ArrayList<>();
        if(found){
            String cur=gk;
            while(!cur.equals(sk)){
                tiles.add(movedTile.get(cur));
                cur=parent.get(cur);
            }
            Collections.reverse(tiles);
        }
        System.out.println("Solved in "+tiles.size()+" moves");
        StringBuilder sb=new StringBuilder("Tiles slid: ");
        for(int i=0;i<tiles.size();i++){
            if(i>0) sb.append(", ");
            sb.append(tiles.get(i));
        }
        System.out.println(sb.toString());
    }
    static class Node{ int f,g; int[] b; Node(int f,int g,int[] b){this.f=f;this.g=g;this.b=b;} }
}
