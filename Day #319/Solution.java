// 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
// Time ~O(b^d) bounded by states explored; Space O(states stored).
import java.util.*;

public class Solution {
    static int manhattan(int[] b){
        int d=0;
        for(int i=0;i<9;i++){
            int v=b[i];
            if(v==0) continue;
            int gr=(v-1)/3, gc=(v-1)%3, r=i/3, c=i%3;
            d+=Math.abs(gr-r)+Math.abs(gc-c);
        }
        return d;
    }
    static String key(int[] b){ return Arrays.toString(b); }

    public static void main(String[] args){
        int[] start={1,2,3,4,5,6,7,0,8};
        int[] goal ={1,2,3,4,5,6,7,8,0};
        String[] names={"Up","Down","Left","Right"};
        int[] delta={-3,3,-1,1};

        Map<String,Integer> g=new HashMap<>();
        Map<String,int[]> parentBoard=new HashMap<>();
        Map<String,String> parentMove=new HashMap<>();
        PriorityQueue<int[]> pq=new PriorityQueue<>((x,y)->Integer.compare(x[0],y[0]));
        Map<Integer,int[]> store=new HashMap<>();
        int idCounter=0;

        g.put(key(start),0);
        store.put(idCounter,start.clone());
        pq.add(new int[]{manhattan(start),0,idCounter++});

        while(!pq.isEmpty()){
            int[] top=pq.poll();
            int gc=top[1];
            int[] cur=store.get(top[2]);
            String ck=key(cur);
            if(gc>g.get(ck)) continue;
            if(ck.equals(key(goal))) break;
            int blank=0; for(int i=0;i<9;i++) if(cur[i]==0) blank=i;
            int r=blank/3,c=blank%3;
            for(int m=0;m<4;m++){
                if(m==0&&r==0) continue;
                if(m==1&&r==2) continue;
                if(m==2&&c==0) continue;
                if(m==3&&c==2) continue;
                int nb=blank+delta[m];
                int[] nx=cur.clone();
                int t=nx[blank]; nx[blank]=nx[nb]; nx[nb]=t;
                String nk=key(nx);
                int ng=gc+1;
                if(!g.containsKey(nk)||ng<g.get(nk)){
                    g.put(nk,ng);
                    parentBoard.put(nk,cur.clone());
                    parentMove.put(nk,names[m]);
                    store.put(idCounter,nx);
                    pq.add(new int[]{ng+manhattan(nx),ng,idCounter++});
                }
            }
        }

        List<String> path=new ArrayList<>();
        String cur=key(goal);
        int[] curB=goal;
        while(!cur.equals(key(start))){
            path.add(parentMove.get(cur));
            curB=parentBoard.get(cur);
            cur=key(curB);
        }
        Collections.reverse(path);

        StringBuilder sb=new StringBuilder();
        sb.append("Solved in ").append(path.size()).append(" move(s): ");
        sb.append(String.join(", ", path));
        System.out.println(sb.toString());
    }
}
