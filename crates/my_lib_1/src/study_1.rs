pub mod fn_methods;
pub mod print_1;
pub mod trait_1;
pub mod trait_2;

#[cfg(test)]
mod tests {
    use crate::study_1::*;

    #[test]
    fn it_works() {
        trait_2::main()
    }
}
