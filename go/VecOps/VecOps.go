package Vecops

import (
	MVec "MVec/Vector"
	"errors"
)

func Hellotheo() string {
	return "Hello, Theo"
}

func Axpy(lhs *MVec.Vector, rhs *MVec.Vector, scalar float32) (MVec.Vector, error) {
	if !SameSize(lhs, rhs) {
		return MVec.Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := MVec.NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i]*scalar + rhs.Components[i]
	}
	return result, nil
}

func Dot(lhs *MVec.Vector, rhs *MVec.Vector) (float32, error) {
	if !SameSize(lhs, rhs) {
		return 0, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	var result float32 = 0
	for i := 0; i < lhs.Size; i++ {
		result += lhs.Components[i] * rhs.Components[i]
	}
	return result, nil
}

func SameSize(lhs *MVec.Vector, rhs *MVec.Vector) bool {
	return lhs.Size == rhs.Size
}

func AddBorrow(lhs *MVec.Vector, rhs *MVec.Vector) (MVec.Vector, error) {
	if !SameSize(lhs, rhs) {
		return MVec.Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := MVec.NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i] + rhs.Components[i]
	}
	return result, nil
}

func SubBorrow(lhs *MVec.Vector, rhs *MVec.Vector) (MVec.Vector, error) {
	if !SameSize(lhs, rhs) {
		return MVec.Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := MVec.NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i] - rhs.Components[i]
	}
	return result, nil
}

func Add(lhs MVec.Vector, rhs MVec.Vector) (MVec.Vector, error) {
	return AddBorrow(&lhs, &rhs)
}

func Sub(lhs MVec.Vector, rhs MVec.Vector) (MVec.Vector, error) {
	return SubBorrow(&lhs, &rhs)
}
