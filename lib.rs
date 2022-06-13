
fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize>{
    let mut history = Vec::new();
    for i in 0..=32 {
        let guess = guesser.guess(&history);
        if guess == answer {
            return i
        }
        let correctness = check(answer, guess);
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.leng(), 5);
        assert_eq!(guess.leng(), 5);    
        let mut c = [Correctness::Wrong; 5];

        for (i, (a,g)) in answers.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }

        for i in 0..5 {
            if answer[i] == guess[i] {
                c[i] = Correctness::Correct;
            }
        }
    }
}


enum Correctness {
    /// Green
    Correct,
    /// yellow
    Present,
    /// Gray
    Missing,
}

struct Guess {
    word: String,
    mask: [Correctness; 5],
}


trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
