// Day 1109: Three-way (Dutch national flag) partition around pivot x.
// Bucket into <x, ==x, >x preserving relative order to match the example.
// Time: O(N). Space: O(N).
#include <bits/stdc++.h>
using namespace std;

vector<int> partitionThree(const vector<int>& lst, int x){
    vector<int> less, equal, greater;
    for (int v : lst){
        if (v < x) less.push_back(v);
        else if (v == x) equal.push_back(v);
        else greater.push_back(v);
    }
    less.insert(less.end(), equal.begin(), equal.end());
    less.insert(less.end(), greater.begin(), greater.end());
    return less;
}

int main(){
    auto r = partitionThree({9,12,3,5,14,10,10}, 10);
    cout << "[";
    for (size_t i=0;i<r.size();i++) cout << r[i] << (i+1<r.size()?", ":"");
    cout << "]\n"; // [9, 3, 5, 10, 10, 12, 14]
    return 0;
}
