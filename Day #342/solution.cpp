// Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

template<typename T, typename A, typename F>
A myReduce(const vector<T>& arr, F f, A init){
    A acc = init;
    for(const T& x : arr) acc = f(acc, x);
    return acc;
}

int add(int a, int b){ return a + b; }

int sum(const vector<int>& lst){
    return myReduce(lst, add, 0);
}

int main(){
    vector<int> lst = {1,2,3,4,5};
    cout << sum(lst) << endl;
    return 0;
}
