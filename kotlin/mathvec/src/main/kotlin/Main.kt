package org.example

//TIP To <b>Run</b> code, press <shortcut actionId="Run"/> or
// click the <icon src="AllIcons.Actions.Execute"/> icon in the gutter.
fun main() {
    val vec1 = Vector(arrayListOf(23.0F, 12.0F, 5.0F, 4.0F, 5.0F))
    val vec2 = Vector(arrayListOf(-2.0F, 4.0F, 50.0F, 6.0F, -34.5F))

    val vec3 = vec1 + vec2
    val vec4 = vec1 - vec2

    println("Vec1: $vec1")
    println("Vec2: $vec2")

    println("Dot Product: ${Vecops.dot(vec1, vec2)}")
    println("AXPY with alpha = -5: ${Vecops.axpy(-5f, vec1, vec2)}")
    println("Vec1 + Vec2 = $vec3")
    println("Vec1 - Vec2 = $vec4")
    println("Vec1 == Vec2 -> ${vec1 == vec2}")

    val vec5 = Vector(arrayListOf(-243.0F, 343.0F, 1.0F, 54.0F, 22.0F, -4.0F))
    println("\nInitial Vec5: $vec5")
    println("Magnitude of Vec5: ${vec5.magnitude()}")
    vec5.normalize()
    println("Normalized Vec5: $vec5")
    println("Magnitude of Vec5: ${vec5.magnitude()}")
}