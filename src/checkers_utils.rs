
pub fn alias_from_coordinates(x: usize, y: usize) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    format!("{}{}", letters[y], 8 - x)
}


pub struct MoveExecutor {

}

impl MoveExecutor {

}