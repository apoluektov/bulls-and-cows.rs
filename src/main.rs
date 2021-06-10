
#[derive(PartialEq, Debug)]
struct BC {
    bulls: u8,
    cows: u8
}


#[derive(PartialEq, Debug)]
struct Code {
    digits: [u8; 4]
}

impl Code {
    fn from_number(n: u16) -> Code {
        let divs = [1000, 100, 10, 1];
        let mut digits = [0 as u8; 4];
        for i in 0..4 {
            digits[i] = ((n / divs[i]) % 10) as u8;
        }

        Code { digits }
    }


    fn is_valid(&self) -> bool {
        let d = &self.digits;

        let mut valid = true;
        for i in 0..4 {
            valid = valid && (d[i] < 10);
            for j in (i+1)..4 {
                valid = valid && (d[i] != d[j]);
            }
        }

        valid
    }
}


fn all_possible_codes() -> Vec<Code> {
    let mut v = Vec::new();
    for n in 0..10000 {
        let code = Code::from_number(n);
        if code.is_valid() {
            v.push(code)
        }
    }

    v
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


struct CodeBreaker {

}


fn main() {
    let mut breaker = CodeBreaker {};


    //println!("{:?}", all_possible_codes());
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

        assert_eq!(Code::from_number(1357), Code { digits: [1,3,5,7] });

        assert!(Code::from_number(1357).is_valid());
        //assert!(!Code::from_number(12345).is_valid()); // FIXME will deal with this later
        assert!(!Code::from_number(1123).is_valid());
    }
}
