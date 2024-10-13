#pragma once
#include <stdexcept>
#include <vector>

namespace MVec
{
    class MVectorException : public std::exception {
        public:
            enum Type {
                SizeMismatch,
                OutOfBounds
            };

            MVectorException(std::string message, Type type) 
                : message{message}, type{type} {}

            const char* what() const noexcept override{
                return message.c_str();
            }

            Type getType(){
                return type;
            }

        private:
            std::string message;
            Type type;
    };

    struct Vector {
        std::vector<float> components;
        size_t size;

        Vector(std::vector<float> array)
            : components{array}, size{array.size()} {}

        void scale(const float scalar);
        const float& at(size_t index) const;
        float& at(size_t index);
        float magnitude();
        void normalize();
        void print();

        bool operator==(Vector const& rhs);
        Vector operator+(Vector const& rhs);
        Vector operator-(Vector const& rhs);
    };

    namespace VecOps{
        // ### Checks if two vectors have the same size
        bool sameSize(const Vector* lhs, const Vector* rhs);
        // ### Returns the axpy
        // defined by: `axpy = (scalar * lhs) + rhs`
        // 
        // > Exceptions: MVectorException
        Vector axpy(const float scalar, const Vector* lhs, const Vector* rhs);
        // ### Returns the dot product between two vectors.
        // Exceptions: MVectorException
        float dot(const Vector* lhs, const Vector* rhs);
    } // namespace VecOps
    

} // namespace MVec
