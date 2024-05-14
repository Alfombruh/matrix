pub struct Matrix {
    pub array: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn from(matrix: &[&[f32]]) -> Self
    {
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
}
