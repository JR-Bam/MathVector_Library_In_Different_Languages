package org.example

import java.util.ArrayList
import kotlin.math.sqrt

class Vector(private var components: ArrayList<Float>) {
    private var size: Int = components.size

    override fun toString(): String {
        return components.toString()
    }

    fun getSize(): Int = size

    fun scale(scalar: Float) {
        for (i in 0 until size) {
            components[i] *= scalar
        }
    }

    // Overloads the [] operator
    operator fun get(index: Int): Float {
        if (index in 0 until size) {
            return components[index]
        } else {
            throw IndexOutOfBoundsException("Index $index")
        }
    }
    operator fun set(index: Int, value: Float) {
        if (index in 0 until size) {
            components[index] = value
        } else {
            throw IndexOutOfBoundsException("Index $index")
        }
    }

    operator fun plus(other: Vector): Vector {
        if (!Vecops.sameSize(this, other))
            throw VectorSizeMismatchException("Vectors should be of same size.")

        val result = ArrayList<Float>()
        for (i in 0 until size){
            result.add(this[i] + other[i])
        }
        return Vector(result)
    }

    operator fun minus(other: Vector): Vector {
        if (!Vecops.sameSize(this, other))
            throw VectorSizeMismatchException("Vectors should be of same size.")

        val copy = Vector(ArrayList(other.components))
        copy.scale(-1F)
        return (this + copy)
    }

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (other !is Vector) return false

        return components == other.components && size == other.size
    }

    override fun hashCode(): Int = 31 * components.hashCode() + size

    fun magnitude(): Float = sqrt(Vecops.dot(this, this))
    fun normalize(): Unit = scale(1F / magnitude())
}