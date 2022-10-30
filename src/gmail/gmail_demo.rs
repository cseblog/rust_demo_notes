extern crate imap;
extern crate native_tls;
extern crate chrono;

use std::str::from_utf8;
use mailparse::*;
use substring::Substring;

use regex::Regex;
use std::ptr::addr_of_mut;
use std::thread::sleep;
use std::time::{Duration, Instant};
use imap::types::Seq;

type ImapSession = imap::Session<native_tls::TlsStream<std::net::TcpStream>>;
type DateTime = chrono::DateTime<chrono::offset::FixedOffset>;


#[derive(Debug)]
struct Message {
    uid: imap::types::Uid,
    body: Vec<u8>,
}

struct Payment {
    name: String,
    amount: String,
    method: String,
    time: String
}

fn get_amount(payment_message: &String) -> String {
    let amount_regex = Regex::new("SGD \\d.* on").unwrap();
    let amount = String::from(amount_regex.find(&payment_message).unwrap().as_str())
        .replace("on", "")
        .replace("SGD ", "")
        .replace(" ", "");
    return amount;
}

fn get_time(payment_message: &String) ->String {
    let time_regex = Regex::new("on.\\d.*T\\)").unwrap();
    let time = String::from(time_regex.find(&payment_message).unwrap().as_str())
        .replace("on", "")
        .replace(" (SGT)", "");
    return time;
}

fn get_name(payment_message: &String) ->String {
    let name_regex = Regex::new("from.*to").unwrap();
    let name = String::from(name_regex.find(&payment_message).unwrap().as_str())
        .replace("from", "").replace("to", "");
    return name;
}

fn get_payment_from_body(email_body: String) -> Payment {
    let reg = Regex::new("You.* PayNow").unwrap();
    let mat = reg.find(&email_body).unwrap().as_str();
    let payment_message = String::from(mat);

    let amount = get_amount(&payment_message);
    let time = get_time(&payment_message);
    let name = get_name(&payment_message);

    return Payment {
        name,
        amount,
        time,
        method: String::from("Paynow"),
    };
}

fn get_messages_by_query(imap_session: &mut ImapSession, query: String) {
    let result = imap_session.search(query);
    let sequences = result.unwrap();
    let mut count = 1;
    for seq in sequences.iter() {
        let messages = get_message_by_id(seq, seq, imap_session);
        for msg in messages.iter() {
            let email = parse_mail(msg.body.as_slice()).unwrap();
            let subject = email.headers.get_first_value("Subject").unwrap();
            let body_string = email.subparts[0].get_body().unwrap();
            let payment = get_payment_from_body(body_string);
            println!("{}: {}, {}$, {}, {}", count, payment.name, payment.amount, payment.time, payment.method );
            count = count + 1;
        }
    }
}

fn get_message_by_id(from: &Seq, to: &Seq, imap_session: &mut ImapSession) -> Vec<Message> {
    let result = imap_session.fetch(format!("{}:{}", from, to), "(UID RFC822)");
    if let Ok(messages) = result {
        return messages.iter().map(|message|
            Message {
                uid: message.uid.unwrap(),
                body: message
                    .body()
                    .map(|x| x.into_iter().map(|&x| x).collect())
                    .unwrap(),
            }
        ).collect::<Vec<_>>();
    }
    vec![]
}

//TODO: Create query Builder class
//TODO: Create Password Manager
fn main() {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        ("imap.gmail.com", 993),
        "imap.gmail.com",
        &tls,
    ).unwrap();

    let username = std::env::var("USERNAME").unwrap();
    let password = std::env::var("PASSWORD").unwrap();

    let mut imap_session = client.login(username, password).unwrap();
    imap_session.select("Inbox").unwrap();

    let from_date = "2-JUL-2022";
    let before_date = "8-JUL-2022";
    let from_email = "ibanking.alert@dbs.com";
    let subject = "Transaction Alerts";
    let query = format!("FROM \"{}\" SUBJECT \"{}\" SINCE \"{}\" BEFORE \"{}\"", from_email, subject, from_date, before_date);
    get_messages_by_query(&mut imap_session, query);
}