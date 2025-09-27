// Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
// Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
import java.util.*;

public class Solution {
    static long[] best = null; // {ax,ay,bx,by}
    static double bestD = Double.MAX_VALUE;

    static double dist(long[] a, long[] b){
        double dx=a[0]-b[0], dy=a[1]-b[1];
        return Math.sqrt(dx*dx+dy*dy);
    }

    // px sorted by x; py sorted by y (same set). returns min distance, updates best.
    static double rec(long[][] px, long[][] py){
        int n = px.length;
        if(n <= 3){
            double d = Double.MAX_VALUE;
            for(int i=0;i<n;i++) for(int j=i+1;j<n;j++){
                double dd = dist(px[i], px[j]);
                if(dd<d){ d=dd; consider(px[i],px[j],dd); }
            }
            return d;
        }
        int mid = n/2;
        long[] midPt = px[mid];
        long[][] lx = Arrays.copyOfRange(px,0,mid);
        long[][] rx = Arrays.copyOfRange(px,mid,n);
        // partition py preserving order
        HashMap<String,Integer> leftCount = new HashMap<>();
        for(long[] p: lx) leftCount.merge(p[0]+","+p[1],1,Integer::sum);
        ArrayList<long[]> lyL=new ArrayList<>(), ryL=new ArrayList<>();
        for(long[] p: py){
            String k=p[0]+","+p[1];
            Integer c=leftCount.get(k);
            if(c!=null && c>0){ lyL.add(p); leftCount.put(k,c-1);} else ryL.add(p);
        }
        long[][] ly=lyL.toArray(new long[0][]); long[][] ry=ryL.toArray(new long[0][]);
        double dl=rec(lx,ly), dr=rec(rx,ry);
        double d=Math.min(dl,dr);
        // strip
        ArrayList<long[]> strip=new ArrayList<>();
        for(long[] p: py) if(Math.abs((double)p[0]-midPt[0])<d) strip.add(p);
        for(int i=0;i<strip.size();i++)
            for(int k=i+1;k<strip.size() && (strip.get(k)[1]-strip.get(i)[1])<d;k++){
                double dd=dist(strip.get(i),strip.get(k));
                if(dd<d){ d=dd; consider(strip.get(i),strip.get(k),dd);}
            }
        return d;
    }

    static void consider(long[] a, long[] b, double d){
        if(d<bestD){ bestD=d; best=new long[]{a[0],a[1],b[0],b[1]}; }
    }

    public static void main(String[] args){
        long[][] pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
        long[][] px = pts.clone();
        Arrays.sort(px,(a,b)-> a[0]!=b[0]? Long.compare(a[0],b[0]):Long.compare(a[1],b[1]));
        long[][] py = pts.clone();
        Arrays.sort(py,(a,b)-> a[1]!=b[1]? Long.compare(a[1],b[1]):Long.compare(a[0],b[0]));
        rec(px,py);
        long ax=best[0],ay=best[1],bx=best[2],by=best[3];
        if(ax>bx || (ax==bx && ay>by)){ long t; t=ax;ax=bx;bx=t; t=ay;ay=by;by=t; }
        System.out.println("[("+ax+", "+ay+"), ("+bx+", "+by+")]");
    }
}
