// Demonstrates the three polymorphism types: ad-hoc (overloading), parametric (generics), subtype (virtual dispatch).
public class Solution {
    // Ad-hoc polymorphism: method overloading.
    static int add(int a, int b) { return a + b; }
    static String add(String a, String b) { return a + b; }

    // Parametric polymorphism: generic method.
    static <T extends Comparable<T>> T maxOf(T a, T b) { return a.compareTo(b) >= 0 ? a : b; }

    // Subtype polymorphism: base class with overridden method.
    static abstract class Shape { abstract double area(); }
    static class Circle extends Shape { double r; Circle(double r){this.r=r;} double area(){ return Math.PI*r*r; } }
    static class Square extends Shape { double s; Square(double s){this.s=s;} double area(){ return s*s; } }

    public static void main(String[] args) {
        System.out.println("Ad-hoc polymorphism: add(2,3)=" + add(2,3)
                + ", add(\"a\",\"b\")=" + add("a","b"));

        System.out.println("Parametric polymorphism: max(3,7)=" + maxOf(3,7)
                + ", max(2.5,1.5)=" + maxOf(2.5,1.5));

        Shape c = new Circle(1.0);
        Shape sq = new Square(2.0);
        System.out.printf("Subtype polymorphism: Circle area=%.5f, Square area=%.1f%n", c.area(), sq.area());
    }
}
