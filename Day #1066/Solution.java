// Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] men   = {"andrew","bill","chester"};
        String[] women = {"abigail","betty","caroline"};
        Map<String,Integer> wi = new HashMap<>(), mi = new HashMap<>();
        for(int i=0;i<3;i++){ wi.put(women[i],i); mi.put(men[i],i); }

        int[][] gp = {
            {wi.get("caroline"),wi.get("abigail"),wi.get("betty")},
            {wi.get("caroline"),wi.get("betty"),wi.get("abigail")},
            {wi.get("betty"),wi.get("caroline"),wi.get("abigail")}
        };
        // gr[w][m] = preference rank of man m for woman w
        int[][] gr = new int[3][3];
        gr[wi.get("abigail")][mi.get("andrew")]=0; gr[wi.get("abigail")][mi.get("bill")]=1; gr[wi.get("abigail")][mi.get("chester")]=2;
        gr[wi.get("betty")][mi.get("bill")]=0;     gr[wi.get("betty")][mi.get("andrew")]=1; gr[wi.get("betty")][mi.get("chester")]=2;
        gr[wi.get("caroline")][mi.get("bill")]=0;  gr[wi.get("caroline")][mi.get("chester")]=1; gr[wi.get("caroline")][mi.get("andrew")]=2;

        int[] wp = new int[3]; Arrays.fill(wp,-1);
        int[] nxt = new int[3];
        Queue<Integer> q = new LinkedList<>();
        for(int i=0;i<3;i++) q.add(i);
        while(!q.isEmpty()){
            int m=q.poll(), w=gp[m][nxt[m]++];
            if(wp[w]==-1) wp[w]=m;
            else if(gr[w][m]<gr[w][wp[w]]){ q.add(wp[w]); wp[w]=m; }
            else q.add(m);
        }
        List<String[]> res = new ArrayList<>();
        for(int w=0;w<3;w++) res.add(new String[]{men[wp[w]], women[w]});
        res.sort((a,b)->a[0].compareTo(b[0]));
        for(String[] p:res) System.out.println(p[0]+" -> "+p[1]);
    }
}
