use std::vec::Vec;
use std::string::String;

pub struct SuffixTable {
    characters : Vec<char>,
    table : Vec<usize>,
}

impl SuffixTable {

    fn index_characters(chars : &Vec<char>) -> (Vec<usize>, bool) {
        let mut tuples : Vec<(char, usize)> = Vec::with_capacity((*chars).len());
        for (i, pc) in (*chars).iter().enumerate() {
            tuples.push((*pc, i));
        }
        tuples.sort();
        let mut table = Vec::with_capacity(tuples.len());
        for _ in tuples.iter() {
            table.push(0);
        }
        let mut index = 0;
        let (mut char_of_this_index, _) = tuples[0];
        for &(c, j) in tuples.iter() {
            if c != char_of_this_index {
                index += 1;
                char_of_this_index = c;
            }
            table[j] = index;
        }
        (table, index == tuples.len()) // the table is ready if no characters are the same
    }

    pub fn new(input : String) -> SuffixTable {
        let chrs : Vec<char> = input.chars().collect();
        let (mut table, done) = SuffixTable::index_characters(&chrs);
        if done {
            SuffixTable{ characters : chrs, table : table }
        } else {
            SuffixTable{ characters : chrs, table : table}
        }
    }

    pub fn find_substr(string : &str) {

    }
}


#[cfg(test)]
mod test {

    use  super::*;

    #[test]
    fn test_all_chars_different() {
        let s_table = SuffixTable::new("cembalo".to_string());
        assert_eq!(s_table.table, vec![2,3,5,1,0,4,6]);
    }
}
