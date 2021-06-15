use std::collections::HashMap;
use std::env;
use std::io;
use std::process;


#[derive(PartialEq, Eq, Debug, Hash)]
struct BC {
    bulls: u8,
    cows: u8
}


#[derive(PartialEq, Debug, Copy, Clone)]
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

    fn make_turn(&mut self, code: &Code) -> BC {
        let bc0 = self.responder.response(&code);
        self.possible_correct_codes.retain(|c| bc(&c, &code) == bc0);

        println!("{:?} -> {:?}", code, bc0);

        bc0
    }

    fn find_best_guess(&self) -> Code {
        if self.possible_guesses.len() == self.possible_correct_codes.len() {
            return self.possible_correct_codes[0];
        }

        if self.possible_correct_codes.len() == 1 {
            return self.possible_correct_codes[0];
        }

        //let mut distribution_by_code = HashMap<Code, HashMap<>>
        let mut worst_case_to_guesses: HashMap<u16, Vec<&Code>> = HashMap::new();
        for guess in self.possible_guesses.iter() {
            let mut response_distribution: HashMap<BC, u16> = HashMap::new();
            for code in self.possible_correct_codes.iter() {
                let resp = bc(guess, code);
                let count = response_distribution.entry(resp).or_insert(0);
                *count += 1;
            }
            assert!(!response_distribution.is_empty());
            let worst_case = response_distribution.values().max().unwrap();
            let guesses = worst_case_to_guesses.entry(*worst_case).or_insert(Vec::new());
            guesses.push(guess);
        }
        assert!(!worst_case_to_guesses.is_empty());
        let (_, best_guesses) = worst_case_to_guesses.iter().min_by_key(|kv| kv.0).unwrap();

        *best_guesses[0]
    }
}

#[derive(Debug)]
enum GameMode {
    HumanBreaksAi,
    AiBreaksHuman,
    AiBreaksAi
}


struct Config {
    mode: GameMode
}

impl Config {
    fn new(args: &[String]) -> Result<Self, String> {
        assert!(args.len() > 0);
        if args.len() > 2 {
            Err(format!("Usage: {} (--human|--ai|--auto)", args[0]))
        } else if args.len() == 2 {
            let mmode = match &args[1][..] {
                "--human" => Ok(GameMode::HumanBreaksAi),
                "--ai" => Ok(GameMode::AiBreaksHuman),
                "--auto" => Ok(GameMode::AiBreaksAi),
                _ => Err(format!("Usage: {} (--human|--ai|--auto)", args[0]))
            };
            mmode.map(|mode| Config { mode })
        } else {
            println!("1. Human breaks the code by AI\n2. AI breaks the code by Human\n3. AI plays with itself.");
            println!("Choose [1-3] (default is 3)");
            let mut opt: String = String::new();
            io::stdin()
                .read_line(&mut opt)
                .expect("Failed to read line");
            let mode = match &opt[..] {
                "1" => GameMode::HumanBreaksAi,
                "2" => GameMode::AiBreaksHuman,
                _ => GameMode::AiBreaksAi
            };
            Ok(Config { mode })
        }
    }
}


fn play_auto() {
    let resp = Responder { secret_code: Code::from_number(1278) };

    let mut breaker = CodeBreaker::new(&resp);
    loop {
        let best_guess = breaker.find_best_guess();
        let res = breaker.make_turn(&best_guess);
        if res.bulls == 4 {
            break;
        }
    }
}


fn play(mode: GameMode) {
    println!("{:?}", mode);
    match mode {
        GameMode::AiBreaksAi => play_auto(),
        _ => panic!("Not implemented yet")
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(cfg) => play(cfg.mode),
        Err(s) => {
            println!("{}", s);
            process::exit(1);
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::Responder;
    use crate::Code;
    use crate::BC;
    use crate::Config;

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

    #[test]
    fn config() {
        let args = vec![String::from("exename"), String::from("--more"), String::from("--options")];
        let cfg = Config::new(&args);
        assert!(cfg.is_err());

        let args = vec![String::from("exename"), String::from("--unknown")];
        let cfg = Config::new(&args);
        assert!(cfg.is_err());

        let args = vec![String::from("exename"), String::from("--auto")];
        let cfg = Config::new(&args);
        assert!(!cfg.is_err());
    }
}
