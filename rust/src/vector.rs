use std::ops;
use std::fmt;

// Define an error type
#[derive(Debug)]
pub enum MVectorError {
    SizeMismatch,
    DirectionMismatch
}

// Implement Display for better error messages
impl fmt::Display for MVectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MVectorError::SizeMismatch => write!(f, "Size mismatch between vectors"),
            MVectorError::DirectionMismatch => write!(f, "Direction mismatch between vectors"),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Vertical,
    Horizontal
}

impl Direction {
    fn flip(&mut self) {
        *self = match *self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        };
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        match self {
            Self::Vertical => Self::Vertical,
            Self::Horizontal => Self::Horizontal,
        }
    }
}

#[derive(Debug)]
pub struct MVector {
    components: Vec<i32>,
    size: usize,
    direction: Direction
}

impl fmt::Display for MVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.components)
    }
}

impl MVector {
    fn same_size(&self, other: &MVector) -> bool {
        self.size == other.size
    }

    fn zip_with<F>(&self, other: &MVector, mut f: F) -> Vec<i32>
    where
        F: FnMut(i32, i32) -> i32,
    {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(&a, &b)| f(a, b)) // Apply the closure `f` on each pair
            .collect()
    }
}


impl MVector {
    pub fn new(numbers: Vec<i32>) -> Self {
        Self {
            size: numbers.len(),
            components: numbers, 
            direction: Direction::Vertical
        }
    }
    pub fn new_zero(size: usize) -> Self {
        Self {
            components: vec![0; size],
            size,
            direction: Direction::Vertical
        }
    }

    pub fn transpose(&mut self) {
        self.direction.flip();
    }

    pub fn scale(&self, alpha: i32) -> MVector {
        let mut new_components = self.components.clone();

        new_components.iter_mut()
            .take(self.size)
            .for_each(|component| {
                *component *= alpha;
            });

        MVector {
            components: new_components,
            size: self.size,
            direction: self.direction.clone()
        }
    }

    pub fn axpy(&self, alpha: i32, other: &MVector) -> Result<MVector, MVectorError>{
        if !self.same_size(other) {
            return Err(MVectorError::SizeMismatch);
        }
        if self.direction != other.direction {
            return Err(MVectorError::DirectionMismatch)
        }
        let scaled_self = self.scale(alpha);
        Ok((&scaled_self + other).unwrap())
    }

    pub fn dot_product(&self, other: &MVector) -> Result<MVector, MVectorError>{
        if !self.same_size(other) {
            return Err(MVectorError::SizeMismatch);
        }
        if self.direction != other.direction {
            return Err(MVectorError::DirectionMismatch)
        }

        Ok(MVector { 
            components: self.zip_with(other, |a, b| a * b),
            size: self.size, 
            direction: self.direction.clone(), 
        })
    }

}

impl<'a> ops::Add<&'a MVector> for &'a MVector {
    type Output = Result<MVector, MVectorError>;

    fn add(self, rhs: &'a MVector) -> Self::Output {
        if !self.same_size(rhs) {
            return Err(MVectorError::SizeMismatch);
        }
        if self.direction != rhs.direction {
            return Err(MVectorError::DirectionMismatch)
        }

        Ok(MVector {
            components: self.zip_with(rhs, |a, b| a + b),
            size: self.size, 
            direction: self.direction.clone(), 
        })
    }
}

impl<'a> ops::Sub<&'a MVector> for &'a MVector {
    type Output = Result<MVector, MVectorError>;

    fn sub(self, rhs: &'a MVector) -> Self::Output {
        if !self.same_size(rhs) {
            return Err(MVectorError::SizeMismatch);
        }
        if self.direction != rhs.direction {
            return Err(MVectorError::DirectionMismatch)
        }

        Ok(MVector {
            components: self.zip_with(rhs, |a, b| a - b),
            size: self.size,
            direction: self.direction.clone()
        })
    }
}
