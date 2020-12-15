use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    account_id: u16,
    district_id: u8,
    frequency: String,
    date: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    card_id: u16,
    disp_id: u16,
    r#type: String,
    issued: String, // date
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    client_id: u16,
    birth_number: String,
    district_id: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Disp {
    disp_id: u16,
    client_id: u16,
    account_id: u16,
    r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct District {
    code: u8,
    name: String,
    region: String,
    inhabitant_number: u32,
    municipalities_inhabitants_0_499: u32,
    municipalities_inhabitants_500_1999: u32,
    municipalities_inhabitants_2000_9999: u32,
    municipalities_inhabitants_10000_inifnity: u32,
    city_numbre: u16,
    ratio_urban_inhabitants: f64,
    average_salary: u32,
    unimployment_rate_95: f64,
    unimployment_rate_96: f64,
    enterpreneurs_per_1000: u16,
    crime_number_95: u32,
    crime_number_96: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Loan {
    loan_id: u16,
    account_id: u16,
    date: String,
    amount: u32,
    duration: u16,
    payments: f64,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    order_id: u16,
    account_id: u16,
    bank_to: String,
    account_to: String,
    amount: f64,
    k_symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Trans {
    trans_id: u32,
    account_id: u16,
    date: String,
    r#type: String,
    operation: String,
    amount: f64,
    balance: f64,
    k_symbol: String,
    bank: String,
    account: String,
}
