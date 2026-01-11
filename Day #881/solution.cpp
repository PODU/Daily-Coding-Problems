// 2D iterator over array of arrays without flattening. next/hasNext amortized O(1).
#include <bits/stdc++.h>
using namespace std;

class Iterator2D {
    const vector<vector<int>>& data;
    size_t row = 0, col = 0;
    void advance(){
        while(row < data.size() && col >= data[row].size()){ row++; col = 0; }
    }
public:
    Iterator2D(const vector<vector<int>>& d): data(d) { advance(); }
    bool hasNext(){ advance(); return row < data.size(); }
    int next(){
        if(!hasNext()) throw runtime_error("no more elements");
        return data[row][col++];
    }
};

int main(){
    vector<vector<int>> arr = {{1, 2}, {3}, {}, {4, 5, 6}};
    Iterator2D it(arr);
    bool first = true;
    while(it.hasNext()){
        if(!first) cout << ", ";
        cout << it.next();
        first = false;
    }
    cout << "\n";
    return 0;
}
