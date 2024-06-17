mod tests;

/// Readonly access to a cell's weight and its immediate neighbours
#[derive(Clone)]
pub struct Neighbourhood<'a, T> {
    pub cell: Option<&'a T>,
    pub neighbours: [Option<&'a T>;6]
}