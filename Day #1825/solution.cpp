// Word circle = Eulerian circuit in graph where each word is an edge first->last char.
// Check in==out degrees, then Hierholzer to build chain. O(V+E).
#include <bits/stdc++.h>
using namespace std;

string solve(vector<string>& words){
    unordered_map<char, vector<pair<string,char>>> adj;
    unordered_map<char,int> indeg, outdeg, idx;
    for(auto& w : words){
        char a = w.front(), b = w.back();
        adj[a].push_back({w, b});
        outdeg[a]++; indeg[b]++;
    }
    set<char> nodes;
    for(auto& p : indeg) nodes.insert(p.first);
    for(auto& p : outdeg) nodes.insert(p.first);
    for(char c : nodes) if(indeg[c] != outdeg[c]) return "No circle";

    char start = words[0].front();
    vector<char> st = {start};
    vector<string> edgeStack, circuit;
    while(!st.empty()){
        char u = st.back();
        if(idx[u] < (int)adj[u].size()){
            auto pr = adj[u][idx[u]++];
            st.push_back(pr.second);
            edgeStack.push_back(pr.first);
        } else {
            st.pop_back();
            if(!edgeStack.empty()){ circuit.push_back(edgeStack.back()); edgeStack.pop_back(); }
        }
    }
    if((int)circuit.size() != (int)words.size()) return "No circle";
    reverse(circuit.begin(), circuit.end());
    string out = circuit[0];
    for(size_t i = 1; i < circuit.size(); i++) out += " --> " + circuit[i];
    out += " --> " + circuit[0];
    return out;
}

int main(){
    vector<string> words = {"chair","height","racket","touch","tunic"};
    cout << solve(words) << "\n";
    return 0;
}
