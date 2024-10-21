#![allow(dead_code)]

use std::fmt::Display;
use std::mem::size_of;
use std::error::Error;
use std::fmt;

struct FooNoDisplay {}

fn display_foo<T: Display>(input: T) {
    println!("{}", input);
}

trait FooTrait{}

enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}

impl FooTrait for EnumOfNumbers {} 

struct StructOfNumbers {
    an_i8: i8,
    anither_i8: i8,
    one_more_i8: i8,
}

impl FooTrait for StructOfNumbers {}

fn returns_foo_trait() -> Box<dyn FooTrait> {
    let enum_of_numbers = EnumOfNumbers::I8(20);
    Box::new(enum_of_numbers)
}

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne{}

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got the first error")
    }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo{}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "You got the second error")
    }
}

fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> {
    match input {
        0 => Err(Box::new(ErrorOne)),
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("I'm okay with this number!".to_string()),
    }
}

fn main() {
    let s = String::from("Coco!");
    display_foo(s);
    let _fnd = FooNoDisplay{};
    //display_foo(fnd);

    println!("{} {}", size_of::<EnumOfNumbers>(), size_of::<StructOfNumbers>());

    let vec_of_u8s = vec![0_u8, 1, 20];
    for elem in vec_of_u8s {
        match returns_errors(elem) {
            Ok(input) => println!("Ok: {}", input),
            Err(message) => println!("Err: {}", message),
        }
    }
}
