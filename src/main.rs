
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


fn bc(c0: &Code, c1: &Code) -> BC {
    let mut bulls = 0;
    let mut cows = 0;
    for (i, &d0) in c0.digits.iter().enumerate() {
        for (j, &d1) in c1.digits.iter().enumerate() {
            if d0 == d1 {
                if i == j {
                    bulls += 1
                } else {
                    cows += 1
                }
            }
        }
    }

    BC {bulls, cows}
}


struct Responder {
    secret_code: Code
}


impl Responder {
    fn response(&self, guess: &Code) -> BC {
        bc(&self.secret_code, guess)
    }
}


struct CodeBreaker<'a> {
    responder: &'a Responder,
    possible_correct_codes: Vec<Code>,
    possible_guesses: Vec<Code>
}


impl<'a> CodeBreaker<'a> {
    fn new(responder: &Responder) -> CodeBreaker {
        CodeBreaker {
            responder,
            possible_correct_codes: all_possible_codes(),
            possible_guesses: all_possible_codes()
        }
    }

    fn make_turn(&mut self, code: Code) -> BC {
        let bc0 = self.responder.response(&code);
        self.possible_correct_codes.retain(|c| bc(&c, &code) == bc0);

        bc0
    }
}


fn main() {
    let resp = Responder { secret_code: Code::from_number(1278) };

    let mut breaker = CodeBreaker::new(&resp);
    breaker.make_turn(Code::from_number(1234));
    breaker.make_turn(Code::from_number(5678));

    println!("{:?}", breaker.possible_correct_codes);


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
