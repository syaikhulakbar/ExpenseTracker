#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data untuk mencatat setiap pengeluaran
#[contracttype]
#[derive(Clone, Debug)]
pub struct Expense {
    id: u64,
    category: String, // Contoh: "Makan", "Transport", "Tagihan Listrik"
    amount: u64,      // Menggunakan u64 agar bisa menampung nominal besar
}

// Storage key untuk data pengeluaran (Maksimal 9 karakter)
const EXP_DATA: Symbol = symbol_short!("EXP_DATA");

#[contract]
pub struct ExpenseTrackerContract;

#[contractimpl]
impl ExpenseTrackerContract {
    // 1. Melihat riwayat semua pengeluaran
    pub fn get_expenses(env: Env) -> Vec<Expense> {
        return env.storage().instance().get(&EXP_DATA).unwrap_or(Vec::new(&env));
    }

    // 2. Menambah catatan pengeluaran baru
    pub fn add_expense(env: Env, category: String, amount: u64) -> String {
        let mut expenses: Vec<Expense> = env.storage().instance().get(&EXP_DATA).unwrap_or(Vec::new(&env));
        
        let new_expense = Expense {
            id: env.prng().gen::<u64>(), // Membuat ID acak
            category: category,
            amount: amount,
        };
        
        expenses.push_back(new_expense);
        
        env.storage().instance().set(&EXP_DATA, &expenses);
        
        return String::from_str(&env, "Pengeluaran berhasil dicatat");
    }

    // 3. Menghitung total semua pengeluaran
    pub fn get_total(env: Env) -> u64 {
        let expenses: Vec<Expense> = env.storage().instance().get(&EXP_DATA).unwrap_or(Vec::new(&env));
        let mut total: u64 = 0;
        
        // Melakukan perulangan (looping) untuk menjumlahkan semua 'amount'
        for i in 0..expenses.len() {
            let expense = expenses.get(i).unwrap();
            total += expense.amount; 
        }
        
        return total;
    }

    // 4. Menghapus riwayat pengeluaran jika ada salah input
    pub fn delete_expense(env: Env, id: u64) -> String {
        let mut expenses: Vec<Expense> = env.storage().instance().get(&EXP_DATA).unwrap_or(Vec::new(&env));

        for i in 0..expenses.len() {
            if expenses.get(i).unwrap().id == id {
                expenses.remove(i); // Hapus data pada index ke-i

                env.storage().instance().set(&EXP_DATA, &expenses);
                return String::from_str(&env, "Berhasil hapus data pengeluaran");
            }
        }

        return String::from_str(&env, "Data pengeluaran tidak ditemukan");
    }
}

mod test;