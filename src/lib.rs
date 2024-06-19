use rand::Rng;
use rand::seq::SliceRandom;

pub fn check_password_size(size: i8){
    if size < 6 {
        panic!("Password size cannot be less than 6");
    }

    if size > 50 {
        panic!("Password size cannot be greater than 50");
    }
}


pub fn generate_password(alphabet: bool, numeric: bool, characters: bool, size: i8) -> String {
    check_password_size(size);

    let mut _vect_resul: Vec<i8> = Vec::new();

    if alphabet && numeric && characters {
        _vect_resul = divide_into_three(size);
    }else if alphabet && numeric || alphabet && characters || numeric && characters {
        _vect_resul = divide_into_two(size);
    }else{
        let mut vec = Vec::with_capacity(1);
        vec.push(size);
        _vect_resul = vec;
    }
    let result_vec = new_random_character(alphabet, numeric, characters, _vect_resul);

    shuffle_string(result_vec)
}

pub fn new_random_character(alphabet: bool, numeric: bool, characters: bool, _vect_resul: Vec<i8>) -> Vec<String>{
    let mut result_vec: Vec<String> = Vec::new();

    if alphabet && numeric && characters{
        for _ in 0.._vect_resul[0]{
            result_vec.push(random_alphabet());
        }
        for _ in 0.._vect_resul[1]{
            result_vec.push(random_number());
        }
        for _ in 0.._vect_resul[2]{
            result_vec.push(random_character());
        }
    } else if alphabet && numeric && !characters {
        for _ in 0.._vect_resul[0]{
            result_vec.push(random_alphabet());
        }
        for _ in 0.._vect_resul[1]{
            result_vec.push(random_number());
        }
    } else if alphabet && !numeric && characters {
        for _ in 0.._vect_resul[0]{
            result_vec.push(random_alphabet());
        }
        for _ in 0.._vect_resul[1]{
            result_vec.push(random_character());
        }
    } else if !alphabet && numeric && characters {
        for _ in 0.._vect_resul[0]{
            result_vec.push(random_number());
        }
        for _ in 0.._vect_resul[1]{
            result_vec.push(random_character());
        }
    } else {
        if alphabet {
            for _ in 0.._vect_resul[0]{
                result_vec.push(random_alphabet());
            }
        }
        if numeric {
            for _ in 0.._vect_resul[0]{
                result_vec.push(random_number());
            }
        }
        if characters {
            for _ in 0.._vect_resul[0]{
                result_vec.push(random_character());
            }
        }
    }
    result_vec
}

pub fn random_alphabet() -> String {
    let _alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut _rng = rand::thread_rng();
    let _random_index = _rng.gen_range(0.._alphabet.len());

    _alphabet.chars().nth(_random_index).unwrap().to_string()
}

pub fn random_number() -> String {
    let mut _rng = rand::thread_rng();
    let _random_number = _rng.gen_range(0..9);

    _random_number.to_string()
}

pub fn random_character() -> String {
    let special_characters = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..special_characters.len());

    special_characters.chars().nth(random_index).unwrap().to_string()
}

fn divide_into_three(number: i8) -> Vec<i8> {
    let mut vec = Vec::with_capacity(3);
    let mut rng = rand::thread_rng();
    let mut num1 = rng.gen_range(1..=number);
    let num2 = rng.gen_range(1..=(number - num1));
    let mut num3 = number - num1 - num2;

    if num3 == 0{
        num1 = num1 - 1;
        num3 = 1;
    }

    vec.push(num1);
    vec.push(num2);
    vec.push(num3);

    vec
}

fn divide_into_two(number: i8) -> Vec<i8> {
    let mut vec = Vec::with_capacity(2);
    let mut rng = rand::thread_rng();

    let num1 = rng.gen_range(1..=number);
    let num2 = number - num1;

    vec.push(num1);
    vec.push(num2);

    vec
}

fn shuffle_string(mut characters_vec: Vec<String>) -> String {
    let mut rng = rand::thread_rng();

    characters_vec.shuffle(&mut rng);
    let result: String = characters_vec.join("");

    result
}
