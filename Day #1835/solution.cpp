// Running median with two heaps (max-heap for lower half, min-heap for upper).
// O(log n) per element.
#include <bits/stdc++.h>
using namespace std;

int main(){
    vector<int> stream = {2, 1, 5, 7, 2, 0, 5};
    priority_queue<int> lo;                                  // lower half (max-heap)
    priority_queue<int, vector<int>, greater<int>> hi;       // upper half (min-heap)

    for(int x : stream){
        if(lo.empty() || x <= lo.top()) lo.push(x); else hi.push(x);
        if(lo.size() > hi.size() + 1){ hi.push(lo.top()); lo.pop(); }
        else if(hi.size() > lo.size()){ lo.push(hi.top()); hi.pop(); }

        double m = (lo.size() > hi.size()) ? lo.top() : (lo.top() + hi.top()) / 2.0;
        if(fabs(m - round(m)) < 1e-9) cout << (long long)llround(m) << "\n";
        else cout << m << "\n";
    }
    return 0;
}
