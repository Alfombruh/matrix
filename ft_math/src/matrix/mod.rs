use crate::vector::Vector;
pub mod ex00;
pub mod ex01;

pub struct Matrix {
    pub array: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn from(matrix: &[&[f32]]) -> Self {
        let mut total: Vec<Vec<f32>> = vec![];
        let len = matrix[0].len();
        for row in matrix {
            if row.len() != len {
                //TO-DO throw exeption
                println!("Matrix has a line with a different size");
            }
            total.push(row.to_vec());
        }
        Self { array: total }
    }
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            array: vec![vec![0.0; columns]; rows],
        }
    }
}

impl Matrix {
    /// Returns the shape of this [`Matrix<T>`] as a tuple (usize, usize).
    /// It does it by just comparing the size of the first row with the ammount of rows,
    /// so it assumes every row has equal lenght
    pub fn shape(&self) -> (usize, usize) {
        return (self.array.len(), self.array[0].len());
    }
    /// Returns true if [`Matrix<T>`]'s shape is squared.
    /// Does assume every row has equal lenght
    pub fn is_square(&self) -> bool {
        if self.shape().0 == self.shape().1 {
            return true;
        }
        return false;
    }
    pub fn print(&self) {
        println!("{:?}", self.array)
    }
    pub fn to_vec(&self) -> Vec<Vector> {
        // Creates an iterator over the matrix's rows
        // maps each row with a function that clones the row and then makes it into a vec
        // collects each row and returns it in a Vec<Vector>
        self.array
            .iter()
            .map(|row| Vector::from_vec(row.clone()))
            .collect()
    }
    pub fn clone(&self) -> Matrix {
        return Matrix { array: self.array.clone() };
    }
}

//MAKES THE CLASS PRINTEABLE WITH JUST {}
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.array)
    }
}