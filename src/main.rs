
#[derive(PartialEq, Debug)]
struct BC {
    bulls: u8,
    cows: u8
}


struct Code {
    digits: [u8; 4]
}


struct Responder {
    secret_code: Code
}


impl Responder {
    fn response(&self, guess: &Code) -> BC {
        let mut bulls = 0;
        let mut cows = 0;
        for (i, &d0) in self.secret_code.digits.iter().enumerate() {
            for (j, &d1) in guess.digits.iter().enumerate() {
                if d0 == d1 {
                    if i == j {
                        bulls += 1
                    } else {
                        cows += 1
                    }
                }
            }
        }
        return BC {bulls, cows}
    }
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::Responder;
    use crate::Code;
    use crate::BC;

    #[test]
    fn it_works() {
        let resp = Responder { secret_code: Code { digits: [1,2,7,8]} };

        let guess = Code { digits: [1,2,3,4] };
        assert_eq!(resp.response(&guess), BC { bulls: 2, cows: 0 });

        let guess = Code { digits: [0,1,7,2] };
        assert_eq!(resp.response(&guess), BC { bulls: 1, cows: 2});
    }
}
