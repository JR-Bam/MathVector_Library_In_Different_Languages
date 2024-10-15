package org.example

import java.util.ArrayList

class VectorSizeMismatchException(message: String) : Exception(message)

object Vecops {
    fun sameSize(lhs: Vector, rhs: Vector): Boolean = lhs.getSize() == rhs.getSize()

    fun axpy(scalar: Float, lhs: Vector, rhs: Vector) : Vector {
        if (!sameSize(lhs, rhs))
            throw VectorSizeMismatchException("Vectors should be of same size.")

        val size = lhs.getSize()
        val result = ArrayList<Float>()

        for (i in 0 until size){
            result.add(lhs[i] * scalar + rhs[i])
        }

        return Vector(result)
    }

    fun dot(lhs: Vector, rhs: Vector) : Float {
        if (!sameSize(lhs, rhs))
            throw VectorSizeMismatchException("Vectors should be of same size.")

        var result = 0F
        for (i in 0 until lhs.getSize()){
            result += lhs[i] * rhs[i]
        }

        return result
    }
}