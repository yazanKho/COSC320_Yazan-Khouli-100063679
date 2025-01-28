// Move Semantics excercises solved          Yazan Khouli        100063679

#![allow(clippy::ptr_arg)]
fn main(){
    /*                  EXERCISE 1              */ 
    // TODO: Fix the compiler error in this function.
    fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut vec = vec;

        vec.push(88);

        vec
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn move_semantics1() {
            let vec0 = vec![22, 44, 66];
            let vec1 = fill_vec(vec0);
            assert_eq!(vec1, vec![22, 44, 66, 88]);
        }
    }


    /*                  EXERCISE 2              */ 
    #[cfg(test)]
    mod tests {
        use super::*;

        // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
        // fix the compiler error in the test.
        #[test]
        fn move_semantics2() {
            let vec0 = vec![22, 44, 66];

            let vec1 = fill_vec(vec0.clone());

            assert_eq!(vec0, [22, 44, 66]);
            assert_eq!(vec1, [22, 44, 66, 88]);
        }
    }

    
    /*                  EXERCISE 3              */ 
    // TODO: Fix the compiler error in the function without adding any new line.
    fn fill_vec2(mut vec: Vec<i32>) -> Vec<i32> {
        vec.push(88);

        vec
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn move_semantics3() {
            let vec0 = vec![22, 44, 66];
            let vec1 = fill_vec2(vec0);
            assert_eq!(vec1, [22, 44, 66, 88]);
        }
    }

    
    /*                  EXERCISE 4              */ 
    #[cfg(test)]
    mod tests {
        // TODO: Fix the compiler errors only by reordering the lines in the test.
        // Don't add, change or remove any line.
        #[test]
        fn move_semantics4() {
            let mut x = Vec::new();
            let y = &mut x;
            y.push(42);
            let z = &mut x;
            z.push(13);
            assert_eq!(x, [42, 13]);
        }
    }


    /*                  EXERCISE 5              */ 

    // TODO: Fix the compiler errors without changing anything except adding or
    // removing references (the character `&`).

    // Shouldn't take ownership
    fn get_char(data: &String) -> char {
        data.chars().last().unwrap()
    }

    // Should take ownership
    fn string_uppercase(mut data: String) {
        data = data.to_uppercase();

        println!("{data}");
    }

    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);


}