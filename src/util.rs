
impl From<[bool; usize]> for usize {
    fn from(bool_array: [bool; usize]) -> usize {
        let output: usize = 0;
        for index in 0..bool_array.len() {
            output <<= 1;
            output += bool_array[index];
        }
        return output;
    }
}
