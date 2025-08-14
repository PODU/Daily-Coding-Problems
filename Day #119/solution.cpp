// Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
// of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
#include <bits/stdc++.h>
using namespace std;
vector<int> minCover(vector<pair<int,int>> iv){
    sort(iv.begin(), iv.end(), [](auto&a, auto&b){ return a.first > b.first; });
    vector<int> pts; bool has = false; int last = 0;
    for(auto& [s,e] : iv){
        if(!has || last > e){ last = s; pts.push_back(s); has = true; }
    }
    sort(pts.begin(), pts.end());
    return pts;
}
int main(){
    auto r = minCover({{0,3},{2,6},{3,4},{6,9}});
    cout << "{";
    for(size_t i=0;i<r.size();++i) cout << r[i] << (i+1<r.size()?", ":"");
    cout << "}\n"; // {3, 6}
    return 0;
}
