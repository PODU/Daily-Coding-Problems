// Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;
int main(){
    vector<pair<int,int>> iv={{1,4},{4,5},{7,9},{9,12}};
    sort(iv.begin(),iv.end(),[](auto&a,auto&b){return a.second<b.second;});
    vector<int> pts; long long last=LLONG_MIN;
    for(auto&p:iv){ if(p.first>last){ last=p.second; pts.push_back(p.second);} }
    cout<<"[";
    for(size_t i=0;i<pts.size();++i){ cout<<pts[i]; if(i+1<pts.size())cout<<", ";}
    cout<<"]\n";
}
