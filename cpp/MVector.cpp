#include "MVector.hpp"
#include <math.h>
#include <iostream>

bool MVec::VecOps::sameSize(Vector const *lhs, Vector const *rhs)
{
    return lhs->size == rhs->size;
}

MVec::Vector MVec::VecOps::axpy(const float scalar, const Vector* lhs, const Vector* rhs)
{
    if (lhs->size != rhs->size)
        throw MVectorException("SizeMismatch: Vectors do not have the same size", MVec::MVectorException::Type::SizeMismatch);
    
    Vector newVec = Vector(lhs->components);

    newVec.scale(scalar);
    for (int i = 0; i < newVec.size; i++){
        newVec.at(i) += rhs->at(i);
    }

    return newVec;
}

float MVec::VecOps::dot(const Vector *lhs, const Vector *rhs)
{
    if (lhs->size != rhs->size)
        throw MVectorException("SizeMismatch: Vectors do not have the same size", MVec::MVectorException::Type::SizeMismatch);
    
    float result = 0;

    for (int i = 0; i < lhs->size; i++){
        result += (lhs->at(i) * rhs->at(i));
    }

    return result;
}

void MVec::Vector::scale(const float scalar)
{
    for (float& component : components){
        component *= scalar;
    }
}

const float &MVec::Vector::at(size_t index) const
{
    if (index < 0 || index >= size)
        throw MVectorException("OutOfBounds: index is out of bounds from Vector", MVec::MVectorException::Type::OutOfBounds);
    
    return components[index];
}

float &MVec::Vector::at(size_t index) {
    if (index < 0 || index >= size)
        throw MVectorException("OutOfBounds: index is out of bounds from Vector", MVec::MVectorException::Type::OutOfBounds);

    return components.at(index);
}

float MVec::Vector::magnitude()
{
    float result = 0;
    for (float& component : components){
        result += (component * component);
    }
    result = sqrt(result);
    return result;
}

void MVec::Vector::normalize()
{
    const float normalizing_constant = magnitude();

    for (float& component : components){
        component /= normalizing_constant;
    }
}

void MVec::Vector::print()
{
    std::cout << "[";

    for (int i = 0; i < size; i++){
        std::cout << at(i);
        std::cout << ((i < size - 1)? ", " : "");
    }

    std::cout << "]\n";
}

bool MVec::Vector::operator==(Vector const &rhs)
{
    if (this->size != rhs.size)
        return false; 
    for (int i = 0; i < this->size; i++){
        if (this->at(i) != rhs.at(i)) return false;
    }
    return true;
}

MVec::Vector MVec::Vector::operator+(Vector const &rhs)
{
    if (this->size != rhs.size)
        throw MVectorException("SizeMismatch: Vectors do not have the same size", MVec::MVectorException::Type::SizeMismatch);
    
    Vector result = Vector(this->components);
    for (int i = 0; i < result.size; i++){
        result.at(i) += rhs.at(i);
    }
    return result;
}

MVec::Vector MVec::Vector::operator-(Vector const &rhs)
{
    if (this->size != rhs.size)
        throw MVectorException("SizeMismatch: Vectors do not have the same size", MVec::MVectorException::Type::SizeMismatch);
    
    Vector result = Vector(this->components);
    for (int i = 0; i < result.size; i++){
        result.at(i) -= rhs.at(i);
    }
    return result;
}
