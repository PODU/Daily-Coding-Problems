// Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
#include <bits/stdc++.h>
using namespace std;

vector<string> findItinerary(vector<pair<string,string>> flights, string start) {
    map<string, multiset<string>> adj;
    for (size_t i = 0; i < flights.size(); i++) adj[flights[i].first].insert(flights[i].second);
    int total = flights.size();
    vector<string> route;
    stack<string> st;
    st.push(start);
    while (!st.empty()) {
        string u = st.top();
        if (!adj[u].empty()) {
            string v = *adj[u].begin();
            adj[u].erase(adj[u].begin());
            st.push(v);
        } else {
            route.push_back(u);
            st.pop();
        }
    }
    reverse(route.begin(), route.end());
    if ((int)route.size() != total + 1) return {}; // not all flights usable
    return route;
}

int main() {
    vector<pair<string,string>> flights = {{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}};
    auto r = findItinerary(flights, "YUL");
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) { if (i) cout << ", "; cout << "'" << r[i] << "'"; }
    cout << "]\n"; // ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
    return 0;
}
