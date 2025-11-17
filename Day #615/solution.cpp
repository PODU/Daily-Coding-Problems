// Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
// Time: O(N^2), Space: O(N^2) for preference rank tables.
#include <bits/stdc++.h>
using namespace std;

int main() {
    map<string, vector<string>> guy = {
        {"andrew",  {"caroline","abigail","betty"}},
        {"bill",    {"caroline","betty","abigail"}},
        {"chester", {"betty","caroline","abigail"}}
    };
    map<string, vector<string>> gal = {
        {"abigail",  {"andrew","bill","chester"}},
        {"betty",    {"bill","andrew","chester"}},
        {"caroline", {"bill","chester","andrew"}}
    };

    // Stable ordering of men matching the example for deterministic output.
    vector<string> men = {"andrew","bill","chester"};

    // rank[woman][man] = preference index (lower = better)
    map<string, map<string,int>> rank;
    for (auto &g : gal)
        for (int i = 0; i < (int)g.second.size(); i++)
            rank[g.first][g.second[i]] = i;

    map<string,int> next_prop;        // next index in man's list to propose to
    for (auto &m : men) next_prop[m] = 0;
    map<string,string> husband;       // woman -> man
    deque<string> freeMen(men.begin(), men.end());

    while (!freeMen.empty()) {
        string m = freeMen.front(); freeMen.pop_front();
        string w = guy[m][next_prop[m]++];
        if (!husband.count(w)) {
            husband[w] = m;
        } else {
            string cur = husband[w];
            if (rank[w][m] < rank[w][cur]) { husband[w] = m; freeMen.push_back(cur); }
            else freeMen.push_back(m);
        }
    }

    map<string,string> wife;
    for (auto &h : husband) wife[h.second] = h.first;

    cout << "{";
    bool first = true;
    for (auto &m : men) {
        if (!first) cout << ", ";
        first = false;
        cout << "'" << m << "': '" << wife[m] << "'";
    }
    cout << "}\n";
    return 0;
}
