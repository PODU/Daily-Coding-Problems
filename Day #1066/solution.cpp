// Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
#include <bits/stdc++.h>
using namespace std;
int main() {
    vector<string> men   = {"andrew","bill","chester"};
    vector<string> women = {"abigail","betty","caroline"};
    map<string,int> wi, mi;
    for(int i=0;i<3;i++) wi[women[i]]=i;
    for(int i=0;i<3;i++) mi[men[i]]=i;

    // guy preferences as woman indices
    vector<vector<int>> gp = {
        {wi["caroline"],wi["abigail"],wi["betty"]},   // andrew
        {wi["caroline"],wi["betty"],wi["abigail"]},   // bill
        {wi["betty"],wi["caroline"],wi["abigail"]}    // chester
    };
    // gr[w][m] = preference rank of man m for woman w (lower = more preferred)
    vector<vector<int>> gr(3, vector<int>(3));
    gr[wi["abigail"]][mi["andrew"]]=0; gr[wi["abigail"]][mi["bill"]]=1; gr[wi["abigail"]][mi["chester"]]=2;
    gr[wi["betty"]][mi["bill"]]=0;     gr[wi["betty"]][mi["andrew"]]=1; gr[wi["betty"]][mi["chester"]]=2;
    gr[wi["caroline"]][mi["bill"]]=0;  gr[wi["caroline"]][mi["chester"]]=1; gr[wi["caroline"]][mi["andrew"]]=2;

    vector<int> wp(3,-1), nxt(3,0);
    queue<int> q;
    for(int i=0;i<3;i++) q.push(i);
    while(!q.empty()){
        int m=q.front(); q.pop();
        int w=gp[m][nxt[m]++];
        if(wp[w]==-1) wp[w]=m;
        else if(gr[w][m]<gr[w][wp[w]]){ q.push(wp[w]); wp[w]=m; }
        else q.push(m);
    }
    vector<pair<string,string>> res;
    for(int w=0;w<3;w++) res.push_back({men[wp[w]], women[w]});
    sort(res.begin(),res.end());
    for(auto& p:res) cout<<p.first<<" -> "<<p.second<<"\n";
}
