// Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
// getInstance() is O(1); instances created lazily once.
#include <bits/stdc++.h>
using namespace std;

class DualSingleton {
    string id;
    DualSingleton(string s):id(s){}
    static long count;
public:
    string name() const { return id; }
    static DualSingleton& getInstance(){
        static DualSingleton first("first");
        static DualSingleton second("second");
        ++count; // 1-based call number
        return (count % 2 == 0) ? first : second; // even -> first, odd -> second
    }
};
long DualSingleton::count = 0;

int main(){
    for(int i = 0; i < 4; ++i)
        cout << DualSingleton::getInstance().name() << "\n"; // second, first, second, first
    cout << boolalpha
         << (&DualSingleton::getInstance() != nullptr) << "\n"; // sanity: returns a valid ref
    return 0;
}
