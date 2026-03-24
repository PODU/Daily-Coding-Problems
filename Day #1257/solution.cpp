// Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
// Time O((N+M) log M), Space O(1).
#include <bits/stdc++.h>
using namespace std;
int minRange(vector<int> listeners,vector<int> towers){
    sort(towers.begin(),towers.end());
    int ans=0;
    for(int l:listeners){
        auto it=lower_bound(towers.begin(),towers.end(),l);
        long long best=LLONG_MAX;
        if(it!=towers.end())best=min(best,(long long)*it-l);
        if(it!=towers.begin())best=min(best,(long long)l-*prev(it));
        ans=max(ans,(int)best);
    }
    return ans;
}
int main(){
    cout<<minRange({1,5,11,20},{4,8,15})<<"\n";
}
