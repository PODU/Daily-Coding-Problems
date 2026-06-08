// Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
// Time O(n), Space O(n).
#include <iostream>
#include <vector>
using namespace std;

vector<int> partition3(int x, const vector<int>& lst) {
    vector<int> less, equal, greater;
    for (int v : lst) {
        if (v < x) less.push_back(v);
        else if (v == x) equal.push_back(v);
        else greater.push_back(v);
    }
    vector<int> res;
    res.insert(res.end(), less.begin(), less.end());
    res.insert(res.end(), equal.begin(), equal.end());
    res.insert(res.end(), greater.begin(), greater.end());
    return res;
}

int main() {
    vector<int> res = partition3(10, {9, 12, 3, 5, 14, 10, 10});
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << res[i];
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
