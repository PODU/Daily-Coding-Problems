// 2D iterator with lazy outer/inner pointers (no flatten/clone).
// next() & hasNext() amortized O(1), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

class Iterator2D {
    const vector<vector<int>>& data;
    size_t outer = 0, inner = 0;
public:
    Iterator2D(const vector<vector<int>>& d) : data(d) {}
    bool hasNext() {
        while (outer < data.size() && inner >= data[outer].size()) { outer++; inner = 0; }
        return outer < data.size();
    }
    int next() {
        if (!hasNext()) throw runtime_error("no more elements");
        return data[outer][inner++];
    }
};

int main() {
    vector<vector<int>> data = {{1, 2}, {3}, {}, {4, 5, 6}};
    Iterator2D it(data);
    bool first = true;
    while (it.hasNext()) {
        if (!first) cout << ", ";
        cout << it.next();
        first = false;
    }
    cout << "\n";
    return 0;
}
