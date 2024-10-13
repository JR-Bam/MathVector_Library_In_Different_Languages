#include <iostream>
#include "MVector.hpp"
using namespace std;

int main(){
    MVec::Vector vec1 = MVec::Vector({23, 12, 5, 4, 5});
    MVec::Vector vec2 = MVec::Vector({-2, 4, 50, 6, -34.5});

    MVec::Vector vec3 = vec1 + vec2;
    MVec::Vector vec4 = vec1 - vec2;

    std::cout << "Vec1: ";
    vec1.print();
    std::cout << "Vec2: ";
    vec2.print();

    std::cout << "Dot Product: " << MVec::dot(&vec1, &vec2) << "\n";
    std::cout << "AXPY with alpha = -5: ";
    MVec::axpy(-5.0f, &vec1, &vec2).print();
    std::cout << "Vec1 + Vec2 = ";
    vec3.print(); 
    std::cout << "Vec1 - Vec2 = ";
    vec4.print(); 
    std::cout << "Vec1 == Vec2 -> " << ((vec1 == vec2)? "true": "false") << '\n';

    MVec::Vector vec5 = MVec::Vector({-243, 343, 1, 54, 22, -4});
    std::cout << "\nInitial Vec5: ";
    vec5.print();
    std::cout << "Magnitude of Vec5: " << vec5.magnitude() << '\n';
    vec5.normalize();
    std::cout << "Normalized Vec5: ";
    vec5.print();
    std::cout << "Magnitude of Vec5: " << vec5.magnitude() << '\n';
    
    return 0;
}