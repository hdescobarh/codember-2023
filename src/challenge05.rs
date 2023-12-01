/*
** The Final Problem **
Finally, hackers have managed to access the database and have corrupted it. But it seems they have left a hidden message in the database. Can you find it?

Our database is in .csv format. The columns are id,username,email,age,location.

A user is only valid if:

- id: exists and is alphanumeric
- username: exists and is alphanumeric
- email: exists and is valid (follows the pattern user@domain.com)
- age: is optional but if it appears it is a number
- location: is optional but if it appears it is a text string
*/

pub fn discover_message(csv_content: &str) -> String {
    let mut message = String::new();
    for line in csv_content.lines() {
        let first = line.chars().next().unwrap();
        message.push(first);
    }
    message
}

// id,username,email,age,location
fn parse_line(line: &str) -> Option<&str> {
    // I suppose that only ASCII are valid (?)
    if !line.is_ascii() {
        return None;
    }

    let splitted_line: Vec<&str> = line.split(',').collect();
    if splitted_line.len() != 5 {
        panic!("Improper line format.")
    }

    // id exists and is alphanumeric
    let id: &str = splitted_line[0];
    if id.is_empty() || id.chars().any(|c| !c.is_alphanumeric()) {
        return None;
    }

    // username exists and is alphanumeric
    let username: &str = splitted_line[1];
    if username.is_empty() || username.chars().any(|c| !c.is_alphanumeric()) {
        return None;
    }

    // age: is optional but if it appears it is a number
    let age: &str = splitted_line[3];
    if !age.is_empty() && age.chars().any(|c| !c.is_numeric()) {
        return None;
    }

    // location is optional but if it appears it is a text string
    let location: &str = splitted_line[4];
    if !location.is_empty() && location.chars().any(|c| !c.is_alphabetic() && c != ' ') {
        return None;
    }

    // email exists and is valid (follows the pattern user@domain.com)
    // validating the patterns without regex is a pain in the ass 😬
    let email: &str = splitted_line[2];
    if !valid_email(email) {
        return None;
    }

    Some(username)
}

// valid email pattern: user@domain.com
fn valid_email(email: &str) -> bool {
    let user_domain: Vec<&str> = email.split('@').collect();
    if user_domain.len() != 2 {
        return false;
    }

    // user is alphanumeric
    if user_domain[0].chars().any(|c| !c.is_alphanumeric()) {
        return false;
    }
    // doesn't accepts subdomains
    let domain: Vec<&str> = user_domain[1].split('.').collect();
    if domain.len() != 2 {
        return false;
    }
    for s in domain {
        if s.chars().any(|c| !c.is_alphanumeric()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    /*
    Examples:

    Entry: 1a421fa,alex,alex9@gmail.com,18,Barcelona
    Result: ✅ Valid

    Entry: 9412p_m,maria,mb@hotmail.com,22,CDMX
    Result: ❌ Invalid (id is not alphanumeric, the _ is extra)

    Entry: 494ee0,madeval,mdv@twitch.tv,,
    Result: ✅ Valid (age and location are optional)
    Entry: 494ee0,madeval,twitch.tv,22,Montevideo
    Result: ❌ Invalid (email is not valid)
    */

    #[test]
    fn line_parser() {
        let test_cases = [
            (Some("alex"), "1a421fa,alex,alex9@gmail.com,18,Barcelona"),
            (None, "9412p_m,maria,mb@hotmail.com,22,CDMX"),
            (Some("madeval"), "494ee0,madeval,mdv@twitch.tv,,"),
            (None, "494ee0,madeval,twitch.tv,22,Montevideo"),
        ];
        for (expected, case) in test_cases {
            assert_eq!(expected, parse_line(case))
        }
    }

    #[test]
    fn email_validation() {
        let test_cases = [
            (true, "alex9@gmail.com"),
            (true, "mb@hotmail.com"),
            (true, "mdv@twitch.tv"),
            (false, "twitch.tv"),
            (false, "alex9"),
            (false, "alex9@twitch"),
        ];

        for (expected, case) in test_cases {
            assert_eq!(expected, valid_email(case))
        }
    }
}
