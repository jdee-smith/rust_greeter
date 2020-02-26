//! This is some crate-level documentation for rust_greeter.
//! 
//! # Examples
//! 
//! ```
//! use rust_greeter::greeter::*;
//! 
//! let greeter: Greeter = Greeter::new(String::from("Jason"));
//! let v: Vec<i8> = vec![3, 6, 9];
//! 
//! assert_eq!(greeter.square(&v), vec![9, 36, 81]);
//! ```

/// This is some module-level documentation for greeter.
#[allow(dead_code)]
pub mod greeter {
    
    pub struct Greeter {
        name: String,
    }

    impl Greeter {
        /// Constructor for a new object of type Greeter.
        /// 
        /// # Examples
        ///
        /// ```
        /// # use rust_greeter::greeter::*;
        /// let greeter: Greeter = Greeter::new(String::from("Jason"));
        /// ```
        pub fn new(name: String) -> Greeter {
            return Greeter { name: name };                              // return keyword is considered
        }                                                               // bad style, but whatevs for now

        pub fn set_name(&mut self, name: String) {
            self.name = name
        }

        pub fn get_name(&self) -> &String {
            return &self.name;
        }

        pub fn say_hello(&self) {
            format!("Hello {}!", self.name);
        }

        pub fn square<T>(&self, v: &Vec<T>) -> Vec<T>
        where
            T: std::ops::Mul<Output = T> + Copy
        {
            let v_size: usize = v.len();
            let mut m: Vec<T> = Vec::with_capacity(v_size);
            for i in v {
                m.push(i.mul(*i));
            }
            return m;
        }
    }

}

#[cfg(test)]
mod tests {
    
    use super::greeter::*;

    #[test]
    fn test_square() {
        let greeter: Greeter = Greeter::new(String::from("Jason"));
        let v: Vec<i8> = vec![1, 2, 3];
        assert_eq!(greeter.square(&v), vec![1, 4, 9]);
    }

}