// Topological sort of courses (Kahn's algorithm with cycle detection).
// Lexicographic tie-break via min-heap for deterministic order.
// Time: O((V+E) log V), Space: O(V+E).
#include <bits/stdc++.h>
using namespace std;

// prereqs[c] = list of courses that must be taken before c.
// Returns empty vector with ok=false if a cycle exists.
vector<string> courseOrder(map<string, vector<string>>& prereqs, bool& ok){
    map<string, vector<string>> adj;   // prereq -> dependents
    map<string, int> indeg;
    for(auto& kv : prereqs){ indeg[kv.first]; } // ensure all nodes present
    for(auto& kv : prereqs){
        for(auto& p : kv.second){
            adj[p].push_back(kv.first);
            indeg[kv.first]++;
            indeg[p];
        }
    }
    priority_queue<string, vector<string>, greater<string>> pq;
    for(auto& kv : indeg) if(kv.second == 0) pq.push(kv.first);
    vector<string> order;
    while(!pq.empty()){
        string c = pq.top(); pq.pop();
        order.push_back(c);
        for(auto& nx : adj[c]) if(--indeg[nx] == 0) pq.push(nx);
    }
    ok = (order.size() == indeg.size());
    if(!ok) order.clear();
    return order;
}

int main(){
    map<string, vector<string>> prereqs = {
        {"CSC300", {"CSC100", "CSC200"}},
        {"CSC200", {"CSC100"}},
        {"CSC100", {}}
    };
    bool ok;
    vector<string> order = courseOrder(prereqs, ok);
    if(!ok){ cout << "null" << endl; return 0; }
    cout << "[";
    for(size_t i=0;i<order.size();i++){ cout << "'" << order[i] << "'"; if(i+1<order.size()) cout << ", "; }
    cout << "]" << endl; // ['CSC100', 'CSC200', 'CSC300']
    return 0;
}
