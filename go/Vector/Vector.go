package Vector

import (
	"errors"
	"fmt"
	"math"
)

type Vector struct {
	Components []float32
	Size       int
}

func (v Vector) String() string {
	return fmt.Sprintf("%v", v.Components)
}

func NewVector(components []float32) Vector {
	return Vector{
		Size:       len(components),
		Components: components,
	}
}

func (v *Vector) Scale(scalar float32) {
	for i := 0; i < len(v.Components); i++ {
		v.Components[i] *= scalar
	}
}

func (v *Vector) AtRef(index int) (*float32, error) {
	if index < 0 || index >= len(v.Components) {
		var zero float32 = 0
		return &zero, errors.New("OutOfBounds: index is out of bounds from Vector")
	}
	return &v.Components[index], nil
}

func (v *Vector) At(index int) (float32, error) {
	if index < 0 || index >= len(v.Components) {
		return 0.0, errors.New("OutOfBounds: index is out of bounds from Vector")
	}
	return v.Components[index], nil
}

func (v *Vector) Magnitude() float32 {
	dot_product, _ := Dot(v, v)
	return float32(math.Sqrt(float64(dot_product)))
}

func (v *Vector) Normalize() {
	v.Scale(1 / v.Magnitude())
}

func Axpy(lhs *Vector, rhs *Vector, scalar float32) (Vector, error) {
	if !SameSize(lhs, rhs) {
		return Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i]*scalar + rhs.Components[i]
	}
	return result, nil
}

func Dot(lhs *Vector, rhs *Vector) (float32, error) {
	if !SameSize(lhs, rhs) {
		return 0, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	var result float32 = 0
	for i := 0; i < lhs.Size; i++ {
		result += lhs.Components[i] * rhs.Components[i]
	}
	return result, nil
}

func SameSize(lhs *Vector, rhs *Vector) bool {
	return lhs.Size == rhs.Size
}

func AddBorrow(lhs *Vector, rhs *Vector) (Vector, error) {
	if !SameSize(lhs, rhs) {
		return Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i] + rhs.Components[i]
	}
	return result, nil
}

func SubBorrow(lhs *Vector, rhs *Vector) (Vector, error) {
	if !SameSize(lhs, rhs) {
		return Vector{}, errors.New("SizeMismatch: Vectors do not have the same size")
	}
	result := NewVector(make([]float32, lhs.Size))
	for i := 0; i < lhs.Size; i++ {
		result.Components[i] = lhs.Components[i] - rhs.Components[i]
	}
	return result, nil
}

func Add(lhs Vector, rhs Vector) (Vector, error) {
	return AddBorrow(&lhs, &rhs)
}

func Sub(lhs Vector, rhs Vector) (Vector, error) {
	return SubBorrow(&lhs, &rhs)
}
