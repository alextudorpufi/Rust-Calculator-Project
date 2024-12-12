use std::io::{self, Write};

#[derive(Debug, Clone)]

// #[allow(dead_code)] // Altfel nu ma lasa sa scriu f64 in Token, char :(

enum Token {
    Number(f64),
    Operator(char),
    LeftPar,
    RightPar,
    LogBase(f64), // de tip logBAZA
    TrigFunc(String),
}

fn lexing(expression: &str) -> Result<Vec<Token>, String /*pt erori */> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();
    let mut last_token_is_number = false;
    let mut last_token_is_operator = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                if last_token_is_number {
                    return Err("Numerele trebuie sa fie separate de operatori.".to_string());
                }
                let mut number_string = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        number_string.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                last_token_is_number = true;
                last_token_is_operator = false;
                tokens.push(Token::Number(number_string.parse::<f64>().unwrap()));
            }
            '+' | '-' | '*' | '/' => {
                if last_token_is_operator {
                    return Err("Operatorii trebuie sa fie separati de numere.".to_string());
                }
                last_token_is_number = false;
                last_token_is_operator = true;
                tokens.push(Token::Operator(ch));
                chars.next();
            }
            '(' => {
                if last_token_is_number {
                    return Err(
                        "Parantezele deschise trebuie sa fie separate de numere.".to_string()
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = false;

                tokens.push(Token::LeftPar);
                chars.next();
            }
            ')' => {
                if last_token_is_operator {
                    return Err(
                        "Parantezele inchise trebuie sa fie separate de operatori.".to_string()
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = false;

                tokens.push(Token::RightPar);
                chars.next();
            }
            ' ' => {
                chars.next();
            }
            '^' => {
                if last_token_is_number {
                    return Err("Operatorii trebuie sa fie separati de numere.".to_string());
                }
                last_token_is_number = false;
                last_token_is_operator = true;

                chars.next();
                chars.next();
                tokens.push(Token::Operator('^'));
            }
            '√' | 'r' => {
                if last_token_is_number {
                    return Err("Operatorii trebuie sa fie separati de numere.".to_string());
                }
                last_token_is_number = false;
                last_token_is_operator = true;

                chars.next();
                tokens.push(Token::Operator('√'));
            }
            'l' => {
                chars.next(); // Consume 'l'
                if chars.next() != Some('o') || chars.next() != Some('g') {
                    return Err(
                        ("Sintaxa incorecta pentru functia log. Asteptam logBAZA.").to_string()
                    );
                }

                let mut base_string = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        base_string.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if last_token_is_operator {
                    return Err(
                        "Functiile trigonometrice trebuie sa fie separate de operatori."
                            .to_string(),
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = true;

                if base_string.is_empty() {
                    tokens.push(Token::LogBase(std::f64::consts::E)); // logaritm natural
                } else {
                    let base = base_string.parse::<f64>().unwrap();
                    tokens.push(Token::LogBase(base));
                }
            }
            's' => {
                chars.next(); // Consuma 's'
                if chars.next() != Some('i') {
                    return Err(
                        "Sintaxa incorecta pentru functia sin. Asteptam sin(x).".to_string()
                    );
                }
                if chars.next() != Some('n') {
                    return Err(
                        "Sintaxa incorecta pentru functia sin. Asteptam sin(x).".to_string()
                    );
                }

                while let Some(&next_char) = chars.peek() {
                    if next_char == ' ' {
                        chars.next();
                    } else {
                        break;
                    }
                }

                if chars.peek() != Some(&'(') {
                    return Err(
                        "Sintaxa incorecta pentru functia sin. Asteptam sin(x).".to_string()
                    );
                }
                if last_token_is_number {
                    return Err(
                        "Functiile trigonometrice trebuie sa fie separate de operatori."
                            .to_string(),
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = true;
                tokens.push(Token::TrigFunc("sin".to_string()));
            }
            'c' => {
                chars.next(); // Consuma 'c'
                if chars.next() != Some('o') {
                    return Err(
                        "Sintaxa incorecta pentru functia cos. Asteptam cos(x).".to_string()
                    );
                }
                if chars.next() != Some('s') {
                    return Err(
                        "Sintaxa incorecta pentru functia cos. Asteptam cos(x).".to_string()
                    );
                }

                while let Some(&next_char) = chars.peek() {
                    if next_char == ' ' {
                        chars.next();
                    } else {
                        break;
                    }
                }

                if chars.peek() != Some(&'(') {
                    return Err(
                        "Sintaxa incorecta pentru functia cos. Asteptam cos(x).".to_string()
                    );
                }

                if last_token_is_number {
                    return Err(
                        "Functiile trigonometrice trebuie sa fie separate de operatori."
                            .to_string(),
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = true;

                tokens.push(Token::TrigFunc("cos".to_string()));
            }
            't' => {
                chars.next();
                if chars.next() != Some('a') {
                    return Err(
                        "Sintaxa incorecta pentru functia tan. Asteptam tan(x).".to_string()
                    );
                }
                if chars.next() != Some('n') {
                    return Err(
                        "Sintaxa incorecta pentru functia tan. Asteptam tan(x).".to_string()
                    );
                }

                while let Some(&next_char) = chars.peek() {
                    if next_char == ' ' {
                        chars.next();
                    } else {
                        break;
                    }
                }

                if chars.peek() != Some(&'(') {
                    return Err(
                        "Sintaxa incorecta pentru functia tan. Asteptam tan(x).".to_string()
                    );
                }

                if last_token_is_number {
                    return Err(
                        "Functiile trigonometrice trebuie sa fie separate de operatori."
                            .to_string(),
                    );
                }
                last_token_is_number = false;
                last_token_is_operator = true;

                tokens.push(Token::TrigFunc("tan".to_string()));
            }
            _ => {
                return Err(format!("Caracter invalid: {}", ch));
            }
        }
    }
    Ok(tokens)
    //fara return pt clippy (@)_(@)
}
fn tokens_to_string(tokens: &[Token]) -> String {
    // pt afisare
    tokens
        .iter()
        .map(|t| match t {
            Token::Number(n) => format!("{:.2}", n),
            Token::Operator(op) => {
                if *op == '^' {
                    "^^".to_string()
                } else {
                    op.to_string()
                }
            }
            Token::LeftPar => "(".to_string(),
            Token::RightPar => ")".to_string(),
            Token::LogBase(base) => format!("log{:.2}", base),
            Token::TrigFunc(functie) => functie.to_string(),
        })
        .collect::<Vec<String>>()
        .join(" ")
}
fn evaluate(tokens: &mut Vec<Token>) -> Result<f64, String> {
    // Printam ecuatia la fiecare pas
    println!("= {}", tokens_to_string(tokens));

    // Paranteze
    while let Some(index) = tokens.iter().position(|t| matches!(t, Token::LeftPar)) {
        let mut right_index = index + 1;
        let mut par_count = 1;

        while par_count > 0 {
            match tokens[right_index] {
                Token::LeftPar => par_count += 1,
                Token::RightPar => par_count -= 1,
                _ => (),
            }
            right_index += 1;
        }

        // Paranteze interioare
        let inner_result = evaluate(&mut tokens[index + 1..right_index - 1].to_vec())?; // ? Pentru propagarea erorilor

        // Inlocuim parantezele cu rezultatul lor
        tokens.splice(index..right_index, vec![Token::Number(inner_result)]);

        println!("= {}", tokens_to_string(tokens));
    }

    // Functii trig
    let mut i = 0;
    while i < tokens.len() {
        if let Token::TrigFunc(functie) = &tokens[i] {
            // Daca gasim o functie trigonometrica, urmatorul token trebuie sa fie un numar, altfel eroare
            if let Token::Number(numar) = tokens[i + 1] {
                if functie == "tan" && numar % 90.0 == 0.0 {
                    return Err(
                        "Tangenta nu poate fi calculata pentru unghiuri de 90, 270, etc."
                            .to_string(),
                    );
                }
                let result = match functie.as_str() {
                    //transformam in radiani pt siguranta
                    "sin" => numar.to_radians().sin(),
                    "cos" => numar.to_radians().cos(),
                    "tan" => numar.to_radians().tan(),
                    _ => unreachable!(),
                };

                tokens.splice(i..=i + 1, vec![Token::Number(result)]);
                println!("= {}", tokens_to_string(tokens));
                continue; // Fara asta, nu rezolva ultima pozitie
            }
        }
        i += 1;
    }

    // Logaritm
    i = 0;
    while i < tokens.len() {
        if let Token::LogBase(base) = tokens[i] {
            if let Token::Number(numar) = tokens[i + 1] {
                if base == 1.0 {
                    return Err("[Baza]Logaritmul nu poate fi calculat pentru baza 1.".to_string());
                }
                if numar <= 0.0 {
                    return Err(
                        "[Valoare]Logaritmul nu poate fi calculat pentru numere negative sau 0."
                            .to_string(),
                    );
                }
                let result = numar.log(base);
                tokens.splice(i..=i + 1, vec![Token::Number(result)]);
                println!("= {}", tokens_to_string(tokens));
                continue;
            }
        }
        i += 1;
    }
    // radical
    i = 0;
    while i < tokens.len() {
        if let Token::Operator(op) = tokens[i] {
            if op == '√' {
                if let Token::Number(numar) = tokens[i + 1] {
                    if numar < 0.0 {
                        return Err(
                            "Radicalul nu poate fi calculat pentru numere negative.".to_string()
                        );
                    }
                    let result = numar.sqrt();
                    tokens.splice(i..=i + 1, vec![Token::Number(result)]);
                    println!("= {}", tokens_to_string(tokens));
                    continue; // Fara asta, nu rezolva ultima pozitie
                }
            }
        }
        i += 1;
    }

    // puteri
    i = 0;
    while i < tokens.len() {
        if let Token::Operator(op) = tokens[i] {
            if op == '^' {
                if let (Token::Number(baza), Token::Number(exponent)) =
                    (&tokens[i - 1], &tokens[i + 1])
                {
                    let result = baza.powf(*exponent);
                    tokens.splice(i - 1..=i + 1, vec![Token::Number(result)]);
                    println!("= {}", tokens_to_string(tokens));
                    continue; // Fara asta, nu rezolva ultima pozitie
                }
            }
        }
        i += 1;
    }

    // inm impartire
    i = 0;
    while i < tokens.len() {
        if let Token::Operator(op) = tokens[i] {
            if op == '*' || op == '/' {
                if let (Token::Number(left), Token::Number(right)) =
                    (&tokens[i - 1], &tokens[i + 1])
                {
                    let result = match op {
                        '*' => left * right,
                        '/' => {
                            if *right == 0.0 {
                                return Err("Impartire la 0.".to_string());
                            }
                            left / right
                        }
                        _ => unreachable!(),
                    };
                    tokens.splice(i - 1..=i + 1, vec![Token::Number(result)]);
                    println!("= {}", tokens_to_string(tokens));
                    continue; // Fara asta, nu rezolva ultima pozitie
                }
            }
        }
        i += 1;
    }

    // adunare scadere
    i = 0;
    while i < tokens.len() {
        if let Token::Operator(op) = tokens[i] {
            if op == '+' || op == '-' {
                if let (Token::Number(left), Token::Number(right)) =
                    (&tokens[i - 1], &tokens[i + 1])
                {
                    let result = match op {
                        '+' => left + right,
                        '-' => left - right,
                        _ => unreachable!(),
                    };
                    tokens.splice(i - 1..=i + 1, vec![Token::Number(result)]);
                    println!("= {}", tokens_to_string(tokens));
                    continue; // Fara asta, nu rezolva ultima pozitie
                }
            }
        }
        i += 1;
    }

    // Daca avem un singur token, inseamna ca am rezolvat tot
    if let Token::Number(n) = tokens[0] {
        Ok(n)
    } else {
        Err("N-am putut evalua expresia".to_string())
        //Cred??
    }
}

fn _old_main() {
    let expression = "(sin(30 + 15) + cos(45)) + log10 100 - √ 4 + 3 + 4 * ( (log2 4 - 1 ) + 3 - 2 ^^ 2) + log (2.72 ^^ 2)";
    println!("Expression: {}", expression);

    // 1. Lexing & Parsing
    let mut tokens = match lexing(expression) {
        Ok(t) => t,
        Err(err) => {
            println!("Lexing error: {}", err);
            return;
        }
    };

    // 2. Resolving
    let result = evaluate(&mut tokens);

    // result
    match result {
        Ok(value) => println!("Final Result: {:.2}", value),
        Err(err) => println!("Error: {}", err),
    }
}

fn print_tutorial() {
    println!("\nGhid pentru utilizarea expresiilor matematice:");
    println!("1. Operatori aritmetici: +, -, *, /, ^");
    println!("2. Functii trigonometrice: sin(x), cos(x), tan(x) (unde x este în grade)");
    println!("3. Logaritmi: logBAZA numar (exemplu: log10 100 sau log 2.72)");
    println!("4. Radical: r numar (exemplu: r4) ( va afisa √4 )");
    println!("5. Paranteze: utilizează ( și ) pentru a controla ordinea operatiilor.");
    println!("Exemplu: (sin(30) + log10 100) / √4");
}

fn main() {
    loop {
        println!("Introdu o expresie matematica,tastati 'help' pentru un ghid sau 'quit'/'exit' pentru a iesi:");

        let mut expression = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut expression)
            .expect("Nu am putut citi expresia.");

        let expression = expression.trim();

        if expression == "help" {
            print_tutorial();
            continue;
        }

        if expression == "exit" || expression == "quit" || expression=="'quit'/'exit'" {
            break;
        }

        println!("Expression: {}", expression);

        // 1. Lexing & Parsing
        let mut tokens = match lexing(expression) {
            Ok(t) => t,
            Err(err) => {
                println!("Lexing error: {}", err);
                return;
            }
        };

        // 2. Resolving
        let result = evaluate(&mut tokens);
  
        // result
        match result {
            Ok(value) => println!("Final Result: {:.2}", value),
            Err(err) => println!("Error: {}", err),
        }
    }
}
