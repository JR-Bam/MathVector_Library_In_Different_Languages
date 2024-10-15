use std::fmt;

// Define an error type
#[derive(Debug)]
pub enum MVectorError {
    SizeMismatch,
    OutOfBounds
}

impl fmt::Display for MVectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MVectorError::SizeMismatch => write!(f, "SizeMismatch: Vectors do not have the same size"),
            MVectorError::OutOfBounds => write!(f, "OutOfBounds: index is out of bounds from Vector"),
        }
    }
}

pub mod mvec {
    use core::fmt;
    use vec_ops::dot;

    use super::MVectorError;

    #[derive(PartialEq)]
    pub struct Vector {
        components: Vec<f32>,
        size: usize
    }

    impl fmt::Display for Vector {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.components)
        }
    }

    impl Vector {
        pub fn new(components: Vec<f32>) -> Self {
            Self { 
                size: components.len(),
                components, 
            }
        }

        pub fn scale(&mut self, scalar: f32){
            for component in self.components.iter_mut() {
                *component *= scalar;
            }
        }

        pub fn at(&self, index: usize) -> Result<f32, MVectorError> {
            if index >= self.size {
                return Err(MVectorError::OutOfBounds);
            }

            Ok(self.components[index])
        }

        pub fn at_ref(&self, index: usize) -> Result<&f32, MVectorError>{
            if index >= self.size {
                return Err(MVectorError::OutOfBounds);
            }

            Ok(&self.components[index])
        }

        pub fn magnitude(&self) -> f32 {
            f32::sqrt(dot(self, self).unwrap())
        }

        pub fn normalize(&mut self){
            self.scale(1.0 / self.magnitude());
        }

    }

    
    // ### Non standard operations on the `mvec::Vector` struct
    pub mod vec_ops {
        use std::ops;

        use crate::vector::MVectorError;
        use super::Vector;

        pub fn axpy(lhs: &Vector, rhs: &Vector, scalar: f32) -> Result<Vector, MVectorError>{
            if !same_size(lhs, rhs) {
                return Err(MVectorError::SizeMismatch);
            }

            let mut result: Vector = Vector::new(lhs.components.clone());
            result.scale(scalar);
            result.components.iter_mut()
                .zip(rhs.components.iter())
                .for_each(|(x, y)| *x += *y);

            Ok(result)
        }
        
        pub fn dot(lhs: &Vector, rhs: &Vector) -> Result<f32, MVectorError> {
            if !same_size(lhs, rhs) {
                return Err(MVectorError::SizeMismatch);
            }

            let mut result: f32 = 0.0;
            lhs.components.iter()
                .zip(rhs.components.iter())
                .for_each(|(x, y)| result += x * y);

            Ok(result)
        }

        pub fn same_size(lhs: &Vector, rhs: &Vector) -> bool {
            lhs.size == rhs.size
        }

        impl<'a, 'b> ops::Add<&'b Vector> for &'a Vector {
            type Output = Result<Vector, MVectorError>;

            fn add(self, rhs: &'b Vector) -> Self::Output {
                if !same_size(self, rhs) {
                    return Err(MVectorError::SizeMismatch);
                }
                let mut sum = Vector::new(self.components.clone());
                sum.components.iter_mut()
                    .zip(rhs.components.iter())
                    .for_each(|(x, y)| *x += *y);
                Ok(sum)
            }
        }

        impl<'a, 'b> ops::Sub<&'b Vector> for &'a Vector {
            type Output = Result<Vector, MVectorError>;
        
            fn sub(self, rhs: &'b Vector) -> Self::Output {
                if !same_size(self, rhs) {
                    return Err(MVectorError::SizeMismatch);
                }
                let mut difference = Vector::new(self.components.clone());
                difference.components.iter_mut()
                    .zip(rhs.components.iter())
                    .for_each(|(x, y)| *x -= *y);
                Ok(difference)
            }
        }

        impl ops::Add for Vector {
            type Output = Result<Vector, MVectorError>;
        
            fn add(self, rhs: Self) -> Self::Output { &self + &rhs }
        }

        impl ops::Sub for Vector {
            type Output = Result<Vector, MVectorError>;
            
            fn sub(self, rhs: Self) -> Self::Output { &self - &rhs }
        }
    }
}
