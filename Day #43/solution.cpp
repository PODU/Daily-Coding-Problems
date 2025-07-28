// Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
// Time O(1) per op; Space O(n).
#include <bits/stdc++.h>
using namespace std;

class MaxStack {
    vector<int> data, maxes;
public:
    void push(int v) {
        data.push_back(v);
        maxes.push_back(maxes.empty() ? v : max(v, maxes.back()));
    }
    // returns top (caller checks empty()); undefined if empty
    int pop() {
        int v = data.back();
        data.pop_back();
        maxes.pop_back();
        return v;
    }
    int max_() const { return maxes.back(); }
    bool empty() const { return data.empty(); }
};

int main() {
    MaxStack s;
    int vals[] = {3, 1, 5, 4};
    for (int v : vals) {
        s.push(v);
        cout << "push " << v << " -> max=" << s.max_() << "\n";
    }
    int p1 = s.pop();
    cout << "pop -> " << p1 << ", max=" << s.max_() << "\n";
    int p2 = s.pop();
    cout << "pop -> " << p2 << ", max=" << s.max_() << "\n";
    return 0;
}
