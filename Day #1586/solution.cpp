// 8-puzzle solver via A* search with Manhattan-distance heuristic (admissible => optimal).
// Time: O(b^d * log) over states explored; Space: O(states) for visited/frontier.
#include <bits/stdc++.h>
using namespace std;

using Board = array<int,9>;

static int manhattan(const Board& b){
    int d=0;
    for(int i=0;i<9;i++){
        int v=b[i];
        if(v==0) continue;
        int goalIdx=v-1; // tile v belongs at index v-1
        d += abs(i/3-goalIdx/3)+abs(i%3-goalIdx%3);
    }
    return d;
}

int main(){
    Board start={1,2,3,4,5,6,0,7,8};
    Board goal ={1,2,3,4,5,6,7,8,0};

    // A*
    map<Board,int> g;            // best g cost
    map<Board,Board> parent;     // for path reconstruction
    map<Board,int> movedTile;    // tile slid to reach this state
    priority_queue<tuple<int,int,Board>, vector<tuple<int,int,Board>>, greater<>> pq;

    g[start]=0;
    pq.push({manhattan(start),0,start});
    parent[start]=start;

    int dr[4]={-1,1,0,0}, dc[4]={0,0,-1,1};
    bool found=false;
    while(!pq.empty()){
        auto [f,gc,cur]=pq.top(); pq.pop();
        if(cur==goal){found=true;break;}
        if(gc>g[cur]) continue;
        int z=0; while(cur[z]!=0) z++;
        int zr=z/3, zc=z%3;
        for(int k=0;k<4;k++){
            int nr=zr+dr[k], nc=zc+dc[k];
            if(nr<0||nr>2||nc<0||nc>2) continue;
            int nz=nr*3+nc;
            Board nb=cur;
            swap(nb[z],nb[nz]);
            int ng=gc+1;
            if(!g.count(nb)||ng<g[nb]){
                g[nb]=ng;
                parent[nb]=cur;
                movedTile[nb]=cur[nz]; // the tile that moved into blank
                pq.push({ng+manhattan(nb),ng,nb});
            }
        }
    }

    vector<int> tiles;
    if(found){
        Board cur=goal;
        while(!(cur==start)){
            tiles.push_back(movedTile[cur]);
            cur=parent[cur];
        }
        reverse(tiles.begin(),tiles.end());
    }
    cout<<"Solved in "<<tiles.size()<<" moves\n";
    cout<<"Tiles slid: ";
    for(size_t i=0;i<tiles.size();i++){
        if(i) cout<<", ";
        cout<<tiles[i];
    }
    cout<<"\n";
    return 0;
}
