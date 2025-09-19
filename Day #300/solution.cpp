// Stream voting: track seen voters + candidate counts; report top-3 (count desc, id asc) or FRAUD on repeat.
// Time: O(n * c log c) over stream, Space: O(voters + candidates).
#include <bits/stdc++.h>
using namespace std;

int main() {
    vector<pair<int,string>> stream = {
        {1,"A"},{2,"B"},{3,"A"},{4,"C"},{5,"B"},{1,"A"},{6,"A"}
    };

    set<int> seen;
    map<string,int> counts; // candidate -> votes

    for (auto &rec : stream) {
        int voter = rec.first;
        string cand = rec.second;
        if (seen.count(voter)) {
            cout << "Fraud: voter " << voter << " voted more than once" << endl;
            continue;
        }
        seen.insert(voter);
        counts[cand]++;

        vector<pair<string,int>> v(counts.begin(), counts.end());
        sort(v.begin(), v.end(), [](const pair<string,int>&a, const pair<string,int>&b){
            if (a.second != b.second) return a.second > b.second; // count desc
            return a.first < b.first; // id asc
        });

        string out = "Top 3: [";
        int limit = min((int)v.size(), 3);
        for (int i = 0; i < limit; ++i) {
            if (i) out += ", ";
            out += "'" + v[i].first + "'";
        }
        out += "]";
        cout << out << endl;
    }
    return 0;
}
