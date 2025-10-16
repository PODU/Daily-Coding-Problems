// Day 440: Demonstrates the three kinds of polymorphism.
// ad-hoc = overloading, parametric = templates, subtype = virtual dispatch.
#include <bits/stdc++.h>
using namespace std;

// Ad-hoc polymorphism: same name, different implementations chosen by type.
int addThem(int a, int b) { return a + b; }
string addThem(const string& a, const string& b) { return a + b; }

// Parametric polymorphism: one definition works for any type.
template <typename T>
T maxOf(T a, T b) { return a > b ? a : b; }

// Subtype polymorphism: base pointer dispatches to derived override.
struct Shape { virtual double area() const = 0; virtual ~Shape() = default; };
struct Circle : Shape { double r; Circle(double r):r(r){} double area() const override { return 3.14159265*r*r; } };
struct Square : Shape { double s; Square(double s):s(s){} double area() const override { return s*s; } };

int main() {
    cout << "Ad-hoc polymorphism (overloading): same name, type-specific impls.\n";
    cout << "  add(2,3) = " << addThem(2,3) << ", add(\"a\",\"b\") = " << addThem(string("a"),string("b")) << "\n";

    cout << "Parametric polymorphism (generics): one impl, any type.\n";
    cout << "  max(3,7) = " << maxOf(3,7) << ", max(\"ab\",\"zz\") = " << maxOf(string("ab"),string("zz")) << "\n";

    cout << "Subtype polymorphism (overriding): base ref, derived behavior.\n";
    vector<unique_ptr<Shape>> shapes;
    shapes.push_back(make_unique<Circle>(1.0));
    shapes.push_back(make_unique<Square>(2.0));
    cout << fixed << setprecision(5);
    for (auto& s : shapes) cout << "  area = " << s->area() << "\n";
    return 0;
}
