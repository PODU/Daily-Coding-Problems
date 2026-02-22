// Day 1114 - Voting stream: top 3 candidates + fraud detection
// Approach: hash-map vote counts + set of seen voters (O(1) dup detection);
// top-3 via partial sort. Time: O(R + M log M), Space: O(V+M).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,string>> stream = {
        {1,"A"},{2,"B"},{3,"A"},{4,"C"},{5,"B"},
        {6,"A"},{2,"C"},{7,"D"},{8,"A"}
    };
    map<string,int> counts;
    set<int> seen;
    for (auto& [voter, cand] : stream) {
        if (seen.count(voter)) {
            cout << "Fraud detected: voter " << voter << " voted more than once\n";
            continue;
        }
        seen.insert(voter);
        counts[cand]++;
    }
    vector<pair<string,int>> items(counts.begin(), counts.end());
    sort(items.begin(), items.end(), [](auto& a, auto& b) {
        if (a.second != b.second) return a.second > b.second;
        return a.first < b.first;
    });
    cout << "Top 3 candidates: [";
    for (int i = 0; i < 3 && i < (int)items.size(); ++i) {
        cout << "('" << items[i].first << "', " << items[i].second << ")";
        if (i + 1 < 3 && i + 1 < (int)items.size()) cout << ", ";
    }
    cout << "]\n";
    return 0;
}
