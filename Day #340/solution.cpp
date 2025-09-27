// Closest pair of points via divide & conquer. O(n log n) time, O(n) space.
// Sort by x, recurse on halves, merge by checking a y-sorted strip near the split.
#include <bits/stdc++.h>
using namespace std;

struct Pt { long long x, y; };

double dist(const Pt&a, const Pt&b){
    double dx=a.x-b.x, dy=a.y-b.y;
    return sqrt(dx*dx+dy*dy);
}

// returns best distance; updates best pair (ba,bb)
double rec(vector<Pt>& px, vector<Pt>& py, Pt& ba, Pt& bb){
    int n = px.size();
    if(n <= 3){
        double best = 1e18;
        for(int i=0;i<n;i++) for(int j=i+1;j<n;j++){
            double d=dist(px[i],px[j]);
            if(d<best){best=d; ba=px[i]; bb=px[j];}
        }
        sort(py.begin(), py.end(), [](const Pt&a,const Pt&b){return a.y<b.y;});
        return best;
    }
    int mid = n/2;
    Pt midPt = px[mid];
    vector<Pt> lx(px.begin(), px.begin()+mid), rx(px.begin()+mid, px.end());
    vector<Pt> ly, ry;
    // partition py preserving y-order; left = the 'mid' smallest-x points (by coordinate+count).
    {
        map<pair<long long,long long>,int> leftCount;
        for(auto&p: lx) leftCount[{p.x,p.y}]++;
        for(auto&p: py){
            auto it=leftCount.find({p.x,p.y});
            if(it!=leftCount.end() && it->second>0){ ly.push_back(p); it->second--; }
            else ry.push_back(p);
        }
    }
    Pt la,lb,ra,rb;
    double dl = rec(lx, ly, la, lb);
    double dr = rec(rx, ry, ra, rb);
    double d;
    if(dl<dr){ d=dl; ba=la; bb=lb; } else { d=dr; ba=ra; bb=rb; }

    // merge py: ly and ry already y-sorted -> merge into py
    {
        vector<Pt> merged; merged.reserve(n);
        int i=0,j=0;
        while(i<(int)ly.size() && j<(int)ry.size()){
            if(ly[i].y<=ry[j].y) merged.push_back(ly[i++]); else merged.push_back(ry[j++]);
        }
        while(i<(int)ly.size()) merged.push_back(ly[i++]);
        while(j<(int)ry.size()) merged.push_back(ry[j++]);
        py = merged;
    }

    // strip
    vector<Pt> strip;
    for(auto&p: py) if(fabs((double)p.x - midPt.x) < d) strip.push_back(p);
    for(int i=0;i<(int)strip.size();i++){
        for(int k=i+1;k<(int)strip.size() && (strip[k].y-strip[i].y)<d; k++){
            double dd=dist(strip[i],strip[k]);
            if(dd<d){ d=dd; ba=strip[i]; bb=strip[k]; }
        }
    }
    return d;
}

int main(){
    vector<Pt> pts = {{1,1},{-1,-1},{3,4},{6,1},{-1,-6},{-4,-3}};
    vector<Pt> px=pts, py=pts;
    sort(px.begin(), px.end(), [](const Pt&a,const Pt&b){return a.x<b.x || (a.x==b.x && a.y<b.y);});
    Pt a,b;
    rec(px, py, a, b);
    // order by x asc then y asc
    if(a.x>b.x || (a.x==b.x && a.y>b.y)) swap(a,b);
    cout << "[(" << a.x << ", " << a.y << "), (" << b.x << ", " << b.y << ")]" << endl;
    return 0;
}
