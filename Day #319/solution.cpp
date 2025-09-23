// 8-puzzle solver: A* search with Manhattan-distance heuristic; blank=0.
// Time ~O(b^d) bounded by states explored; Space O(states stored).
#include <bits/stdc++.h>
using namespace std;

using Board = array<int,9>;

int manhattan(const Board& b){
    int d=0;
    for(int i=0;i<9;i++){
        int v=b[i];
        if(v==0) continue;
        int gr=(v-1)/3, gc=(v-1)%3;
        int r=i/3, c=i%3;
        d+=abs(gr-r)+abs(gc-c);
    }
    return d;
}

int main(){
    Board start={1,2,3,4,5,6,7,0,8};
    Board goal ={1,2,3,4,5,6,7,8,0};

    // directions: blank moves Up/Down/Left/Right
    vector<pair<int,string>> moves={{-3,"Up"},{3,"Down"},{-1,"Left"},{1,"Right"}};

    map<Board,int> g;          // cost so far
    map<Board,pair<Board,string>> parent;
    priority_queue<tuple<int,int,Board>, vector<tuple<int,int,Board>>, greater<>> pq;

    g[start]=0;
    pq.push({manhattan(start),0,start});

    while(!pq.empty()){
        tuple<int,int,Board> top=pq.top(); pq.pop();
        int gc=get<1>(top);
        Board cur=get<2>(top);
        if(gc>g[cur]) continue;
        if(cur==goal) break;
        int blank=find(cur.begin(),cur.end(),0)-cur.begin();
        int r=blank/3,c=blank%3;
        for(auto& m:moves){
            int nb=blank+m.first;
            if(m.second=="Up" && r==0) continue;
            if(m.second=="Down" && r==2) continue;
            if(m.second=="Left" && c==0) continue;
            if(m.second=="Right" && c==2) continue;
            Board nx=cur; swap(nx[blank],nx[nb]);
            int ng=gc+1;
            if(!g.count(nx)||ng<g[nx]){
                g[nx]=ng;
                parent[nx]=make_pair(cur,m.second);
                pq.push({ng+manhattan(nx),ng,nx});
            }
        }
    }

    // reconstruct
    vector<string> path;
    Board cur=goal;
    while(cur!=start){
        auto& p=parent[cur];
        path.push_back(p.second);
        cur=p.first;
    }
    reverse(path.begin(),path.end());

    cout<<"Solved in "<<path.size()<<" move(s): ";
    for(size_t i=0;i<path.size();i++){
        if(i) cout<<", ";
        cout<<path[i];
    }
    cout<<"\n";
    return 0;
}
