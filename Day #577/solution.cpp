// Word circle: model words as directed edges first->last char; Eulerian circuit via Hierholzer. Time O(V+E), Space O(V+E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<string> words = {"chair", "height", "racket", "touch", "tunic"};
    // adjacency: char -> list of (nextChar, word) in input order
    map<char, vector<pair<char, string>>> adj;
    for (auto &w : words)
        adj[w.front()].push_back({w.back(), w});
    map<char, size_t> ptr;

    char start = words[0].front();
    vector<string> path;
    // stack frames: (char, word used to enter, or empty for start)
    vector<pair<char, string>> st;
    st.push_back({start, ""});
    while (!st.empty()) {
        char v = st.back().first;
        if (ptr[v] < adj[v].size()) {
            auto edge = adj[v][ptr[v]++];
            st.push_back({edge.first, edge.second});
        } else {
            string w = st.back().second;
            st.pop_back();
            if (!w.empty()) path.push_back(w);
        }
    }
    reverse(path.begin(), path.end());

    string out;
    for (auto &w : path) out += w + " --> ";
    out += path.front();
    cout << out << "\n";
    return 0;
}
