// Brick wall: prefix-sum edge positions per row, max edge frequency => fewest cuts = rows - maxEdges. O(total bricks) time/space.
#include <bits/stdc++.h>
using namespace std;
int main(){
    vector<vector<int>> wall={{3,5,1,1},{2,3,3,2},{5,5},{4,4,2},{1,3,3,3},{1,1,6,1,1}};
    unordered_map<long long,int> freq;
    int best=0;
    for(auto& row:wall){
        long long sum=0;
        for(size_t i=0;i+1<row.size();++i){ sum+=row[i]; best=max(best,++freq[sum]); }
    }
    cout<<(int)wall.size()-best<<"\n";
    return 0;
}
