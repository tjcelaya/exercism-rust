extern crate itertools;
use itertools::Itertools;

const SURE: &'static str = "Sure.";
const WHOA: &'static str = "Whoa, chill out!";
const WHATEVER: &'static str = "Whatever.";
const CALM_DOWN: &'static str = "Calm down, I know what I'm doing!";
const FINE: &'static str = "Fine. Be that way!";
const WATCH_OUT: &'static str = "WATCH OUT!";



pub fn reply(message: &str) -> &str {

    let mut chars = message.chars();
    // let reversed = chars.rev();

    let punc = reversed.take_while_ref(|c| c.eq_ignore_ascii_case(&'.') || c.eq_ignore_ascii_case(&'?') || c.eq_ignore_ascii_case(&'!'));

    println!("next is [{:?}]", reversed.next());


    return WHATEVER;
}

/*
Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question.

He answers 'Whoa, chill out!' if you yell at him.

He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else.

Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.
*/
