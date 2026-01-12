// Dutch national flag: three-pointer in-place partition R<G<B. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

void sortRGB(vector<char>& a){
    int low = 0, mid = 0, high = a.size() - 1;
    while(mid <= high){
        if(a[mid] == 'R'){ swap(a[low++], a[mid++]); }
        else if(a[mid] == 'G'){ mid++; }
        else { swap(a[mid], a[high--]); } // 'B'
    }
}

int main(){
    vector<char> a = {'G', 'B', 'R', 'R', 'B', 'R', 'G'};
    sortRGB(a);
    cout << "[";
    for(size_t i = 0; i < a.size(); i++) cout << "'" << a[i] << "'" << (i + 1 < a.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
