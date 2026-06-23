// Stream voting: hashmap vote counts + set of seen voters; duplicate voter = fraud.
// Top 3 by count (ties by candidate id). Time O(n + k log k), Space O(k+v).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,string>> records = {{1,"A"},{2,"B"},{3,"A"},{4,"C"},{2,"A"},{5,"B"},{6,"A"}};
    unordered_map<string,int> counts;
    unordered_set<int> seen;
    for (auto &r : records) {
        if (seen.count(r.first)) {
            cout << "Fraud detected: voter " << r.first << " voted more than once\n";
            continue;
        }
        seen.insert(r.first);
        counts[r.second]++;
    }
    vector<pair<string,int>> v(counts.begin(), counts.end());
    sort(v.begin(), v.end(), [](auto &a, auto &b){
        if (a.second != b.second) return a.second > b.second;
        return a.first < b.first;
    });
    cout << "Top 3 candidates: ";
    for (int i = 0; i < 3 && i < (int)v.size(); i++) {
        if (i) cout << ", ";
        cout << v[i].first << "(" << v[i].second << ")";
    }
    cout << "\n";
    return 0;
}
