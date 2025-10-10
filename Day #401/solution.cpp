// Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
// (In-place alternative: follow cycles swapping elements.)
#include <iostream>
#include <vector>
#include <string>
using namespace std;

vector<string> applyPermutation(const vector<string>& array, const vector<int>& P) {
    vector<string> result(array.size());
    for (size_t i = 0; i < array.size(); ++i) result[P[i]] = array[i];
    return result;
}

int main() {
    vector<string> array = {"a", "b", "c"};
    vector<int> P = {2, 1, 0};
    vector<string> res = applyPermutation(array, P);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i) {
        cout << "\"" << res[i] << "\"";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
