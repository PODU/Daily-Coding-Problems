// Max stack with O(1) push/pop/max.
// Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).
#include <bits/stdc++.h>
using namespace std;

// Result wrapper: ok=false means the stack was empty (null).
struct Result {
    int value;
    bool ok;
};

class MaxStack {
    vector<int> data, maxs;
public:
    void push(int val) {
        data.push_back(val);
        maxs.push_back(maxs.empty() ? val : max(val, maxs.back()));
    }
    // returns top, or {.ok=false} if empty
    Result pop() {
        if (data.empty()) return {0, false};
        int v = data.back();
        data.pop_back(); maxs.pop_back();
        return {v, true};
    }
    Result max_() const {
        if (maxs.empty()) return {0, false};
        return {maxs.back(), true};
    }
};

static string show(Result r) { return r.ok ? to_string(r.value) : "null"; }

int main() {
    MaxStack s;
    s.push(3); s.push(1); s.push(5); s.push(2);
    cout << "max: " << show(s.max_()) << "\n";
    cout << "pop: " << show(s.pop()) << "\n";
    cout << "pop: " << show(s.pop()) << "\n";
    cout << "max: " << show(s.max_()) << "\n";
    return 0;
}
