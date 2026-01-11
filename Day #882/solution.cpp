// 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
// State = 9-char string, 0 = blank. Time/space depend on solution depth.
#include <bits/stdc++.h>
using namespace std;

static const string GOAL = "123456780";

int manhattan(const string& s){
    int d = 0;
    for(int i = 0; i < 9; i++){
        if(s[i] == '0') continue;
        int v = s[i] - '1';
        d += abs(i / 3 - v / 3) + abs(i % 3 - v % 3);
    }
    return d;
}

vector<int> solve(const string& start){
    priority_queue<tuple<int,int,string>, vector<tuple<int,int,string>>, greater<>> pq;
    unordered_map<string,int> g;
    unordered_map<string,pair<string,int>> parent;
    g[start] = 0;
    pq.push(make_tuple(manhattan(start), 0, start));
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
    while(!pq.empty()){
        tuple<int,int,string> top = pq.top(); pq.pop();
        int gc = get<1>(top);
        string cur = get<2>(top);
        if(cur == GOAL) break;
        if(gc > g[cur]) continue;
        int z = cur.find('0'), r = z / 3, c = z % 3;
        for(int k = 0; k < 4; k++){
            int nr = r + dr[k], nc = c + dc[k];
            if(nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
            int nz = nr * 3 + nc;
            string nxt = cur; swap(nxt[z], nxt[nz]);
            int ng = gc + 1;
            if(!g.count(nxt) || ng < g[nxt]){
                g[nxt] = ng;
                parent[nxt] = make_pair(cur, (int)(cur[nz] - '0'));
                pq.push(make_tuple(ng + manhattan(nxt), ng, nxt));
            }
        }
    }
    vector<int> moves;
    string cur = GOAL;
    while(cur != start){
        auto& p = parent[cur];
        moves.push_back(p.second);
        cur = p.first;
    }
    reverse(moves.begin(), moves.end());
    return moves;
}

int main(){
    // Start: [[1,2,3],[4,_,6],[7,5,8]] (blank in center)
    string start = "123406758";
    vector<int> moves = solve(start);
    cout << "Solved in " << moves.size() << " moves: [";
    for(size_t i = 0; i < moves.size(); i++) cout << moves[i] << (i + 1 < moves.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
