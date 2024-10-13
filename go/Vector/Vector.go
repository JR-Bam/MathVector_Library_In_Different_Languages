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
	var result float32 = 0
	for i := 0; i < len(v.Components); i++ {
		component, _ := v.At(i)
		result += component * component
	}
	return float32(math.Sqrt(float64(result)))
}

func (v *Vector) Normalize() {
	normalizing_constant := v.Magnitude()
	for i := 0; i < len(v.Components); i++ {
		v.Components[i] /= normalizing_constant
	}
}
