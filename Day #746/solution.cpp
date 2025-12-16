// Max stack: each entry stores the running max so far, giving O(1) push/pop/max.
// Time: O(1) per operation, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

class MaxStack {
    vector<pair<int,int>> st; // {value, maxSoFar}
public:
    void push(int v){
        int mx = st.empty() ? v : std::max(v, st.back().second);
        st.push_back({v, mx});
    }
    // returns {ok, value}
    pair<bool,int> pop(){
        if(st.empty()) return {false, 0};
        int v = st.back().first; st.pop_back();
        return {true, v};
    }
    pair<bool,int> max(){
        if(st.empty()) return {false, 0};
        return {true, st.back().second};
    }
};

int main(){
    MaxStack s;
    s.push(1); s.push(3); s.push(2);
    cout << "max: " << s.max().second << "\n";  // 3
    cout << "pop: " << s.pop().second << "\n";  // 2
    cout << "max: " << s.max().second << "\n";  // 3
    cout << "pop: " << s.pop().second << "\n";  // 3
    cout << "max: " << s.max().second << "\n";  // 1
    return 0;
}
