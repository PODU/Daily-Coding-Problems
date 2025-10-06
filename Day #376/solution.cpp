// Closest coin by Manhattan distance. Linear scan.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

pair<int,int> closestCoin(pair<int,int> me, const vector<pair<int,int>>& coins){
    pair<int,int> best{-1,-1};
    long bestD = LONG_MAX;
    for(const auto& c : coins){
        long d = labs(c.first - me.first) + labs(c.second - me.second);
        if(d < bestD){ bestD = d; best = c; }
    }
    return best;
}

int main(){
    pair<int,int> me{0,2};
    vector<pair<int,int>> coins{{0,4},{1,0},{2,0},{3,2}};
    auto b = closestCoin(me, coins);
    cout << "(" << b.first << ", " << b.second << ")\n";
    return 0;
}
