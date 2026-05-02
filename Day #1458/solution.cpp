// Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
// Time: O(100*6). Space: O(100).
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<int,int> jump = {
        {16,6},{48,26},{49,11},{56,53},{62,19},{64,60},{87,24},{93,73},{95,75},{98,78}, // snakes
        {1,38},{4,14},{9,31},{21,42},{28,84},{36,44},{51,67},{71,91},{80,100}           // ladders
    };
    auto land = [&](int s){ auto it = jump.find(s); return it==jump.end()? s : it->second; };

    int start = land(1);
    vector<int> dist(101, -1);
    queue<int> q;
    dist[start] = 0;
    q.push(start);
    int ans = -1;
    while(!q.empty()){
        int s = q.front(); q.pop();
        if(s == 100){ ans = dist[s]; break; }
        for(int d=1; d<=6; ++d){
            int nxt = s + d;
            if(nxt > 100) continue;
            nxt = land(nxt);
            if(dist[nxt] == -1){
                dist[nxt] = dist[s] + 1;
                q.push(nxt);
            }
        }
    }
    cout << ans << "\n";
    return 0;
}
