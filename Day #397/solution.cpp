// Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
// Time O(n log n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

vector<pair<int,int>> selectJobs(vector<pair<int,int>> jobs) {
    sort(jobs.begin(), jobs.end(), [](const pair<int,int>& a, const pair<int,int>& b) {
        return a.second < b.second;
    });
    vector<pair<int,int>> chosen;
    int lastEnd = INT_MIN;
    for (auto& j : jobs) {
        if (j.first >= lastEnd) {
            chosen.push_back(j);
            lastEnd = j.second;
        }
    }
    return chosen;
}

int main() {
    vector<pair<int,int>> jobs = {{0,6},{1,4},{3,5},{3,8},{4,7},{5,9},{6,10},{8,11}};
    auto chosen = selectJobs(jobs);
    cout << "[";
    for (size_t i = 0; i < chosen.size(); ++i) {
        cout << "(" << chosen[i].first << ", " << chosen[i].second << ")";
        if (i + 1 < chosen.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
