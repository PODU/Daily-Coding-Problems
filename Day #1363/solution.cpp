// Stream voting: HashMap candidate->count, HashSet of voters to detect fraud; top-3 via sort.
// Time: O(records) processing + O(C log C) reporting. Space: O(C + V).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<string,string>> stream = {
        {"v1","A"},{"v2","B"},{"v3","A"},{"v4","C"},
        {"v5","B"},{"v6","A"},{"v7","C"},{"v1","B"}
    };

    unordered_map<string,int> counts;
    unordered_set<string> seen;

    for (auto& rec : stream) {
        const string& voter = rec.first;
        const string& cand = rec.second;
        if (seen.count(voter)) {
            cout << "Fraud detected: voter " << voter << "\n";
            continue;
        }
        seen.insert(voter);
        counts[cand]++;
    }

    vector<pair<string,int>> v(counts.begin(), counts.end());
    sort(v.begin(), v.end(), [](const pair<string,int>& a, const pair<string,int>& b){
        if (a.second != b.second) return a.second > b.second;
        return a.first < b.first;
    });

    cout << "Top 3 candidates: ";
    for (size_t i = 0; i < v.size() && i < 3; ++i) {
        if (i) cout << ", ";
        cout << v[i].first;
    }
    cout << "\n";
    return 0;
}
