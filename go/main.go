package main

import (
	MVec "MVec/Vector"
	"fmt"
)

func main() {
	vec1 := MVec.NewVector([]float32{23.0, 12.0, 5.0, 4.0, 5.0})
	vec2 := MVec.NewVector([]float32{-2.0, 4.0, 50.0, 6.0, -34.5})

	vec3, _ := MVec.Add(vec1, vec2) // Copy Semantics
	vec4, _ := MVec.Sub(vec1, vec2) // Copy Semantics

	fmt.Println("Vec1:", vec1)
	fmt.Println("Vec2:", vec2)
	dotProduct, _ := MVec.Dot(&vec1, &vec2)
	fmt.Println("Dot Product:", dotProduct)
	axpyResult, _ := MVec.Axpy(&vec1, &vec2, -5)
	fmt.Println("AXPY with alpha = -5:", axpyResult)
	fmt.Println("Vec1 + Vec2 =", vec3)
	fmt.Println("Vec1 - Vec2 =", vec4)
	fmt.Println("Vec1 == Vec2 ->", MVec.SameSize(&vec1, &vec2))

	vec5 := MVec.NewVector([]float32{-243.0, 343.0, 1.0, 54.0, 22.0, -4.0})
	fmt.Println("\nInitial Vec5:", vec5)
	fmt.Println("Magnitude of Vec5:", vec5.Magnitude())
	vec5.Normalize()
	fmt.Println("Normalized Vec5:", vec5)
	fmt.Println("Magnitude of Vec5:", vec5.Magnitude())
}
