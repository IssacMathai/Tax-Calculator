use wasm_bindgen::prelude::*; // For WASM interaction

#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    let mut tax = 0.0;

    if income <= 9875.0 {
        tax = income * 0.10;
    } else if income <= 40125.0 {
        tax = 987.50 + (income - 9875.0) * 0.12;
    } else if income <= 85525.0 {
        tax = 4617.50 + (income - 40125.0) * 0.22;
    } else if income <= 207350.0 {
        tax = 33271.50 + (income - 163300.0) * 0.32;
    } else if income <= 518400.0 {
        tax = 47367.5 + (income - 207350.0) * 0.35;
    } else {
        tax = 156235.0 + (income - 518400.0) * 0.37
    }

    return tax
}