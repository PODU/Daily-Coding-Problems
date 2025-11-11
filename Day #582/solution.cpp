// Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

vector<int> stab(vector<pair<int,int>> intervals) {
    sort(intervals.begin(), intervals.end(),
         [](const pair<int,int>& a, const pair<int,int>& b){ return a.second < b.second; });
    vector<int> points;
    long long last = LLONG_MIN;
    for (auto& iv : intervals) {
        if (iv.first > last) {
            last = iv.second;
            points.push_back(iv.second);
        }
    }
    return points;
}

int main() {
    vector<pair<int,int>> intervals = {{1,4},{4,5},{7,9},{9,12}};
    vector<int> pts = stab(intervals);
    cout << "[";
    for (size_t i = 0; i < pts.size(); ++i) {
        cout << pts[i];
        if (i + 1 < pts.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
