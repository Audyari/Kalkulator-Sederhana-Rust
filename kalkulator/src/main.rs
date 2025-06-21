use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Deklarasi variabel
    let mut angka_pertama: f64;      // MUTABLE - nilainya bisa diubah
    let mut angka_kedua: f64;        // MUTABLE - nilainya bisa diubah
    let mut hasil: f64;              // MUTABLE - akan menyimpan hasil perhitungan
    let mut _pesan_error = String::new();    // MUTABLE - untuk menampung pesan error
    let mut _lanjut = true;                 // MUTABLE - untuk kontrol loop
    let mut _input_valid: bool;             // MUTABLE - untuk validasi input

    loop {
        clear_screen();
        menu();
        
        // Reset variabel
        _input_valid = false;               // MUTABLE - diubah nilainya
        let mut pilihan = String::new();   // MUTABLE - untuk menyimpan input pengguna
        
        // Baca pilihan operasi
        print!("\nPilih operasi (1-5): ");
        io::stdout().flush().unwrap();
        
        match io::stdin().read_line(&mut pilihan) {
            Ok(_) => {
                match pilihan.trim() {
                    "1" => {
                        // Penjumlahan
                        angka_pertama = baca_angka("Masukkan angka pertama: ");
                        angka_kedua = baca_angka("Masukkan angka kedua: ");
                        hasil = tambah(angka_pertama, angka_kedua);
                        println!("\nHasil {} + {} = {}", angka_pertama, angka_kedua, hasil);
                        pause();
                    },
                    "2" => {
                        // Pengurangan
                        angka_pertama = baca_angka("Masukkan angka pertama: ");
                        angka_kedua = baca_angka("Masukkan angka kedua: ");
                        hasil = kurang(angka_pertama, angka_kedua);
                        println!("\nHasil {} - {} = {}", angka_pertama, angka_kedua, hasil);
                        pause();
                    },
                    "3" => {
                        // Perkalian
                        angka_pertama = baca_angka("Masukkan angka pertama: ");
                        angka_kedua = baca_angka("Masukkan angka kedua: ");
                        hasil = kali(angka_pertama, angka_kedua);
                        println!("\nHasil {} * {} = {}", angka_pertama, angka_kedua, hasil);
                        pause();
                    },
                    "4" => {
                        // Pembagian
                        angka_pertama = baca_angka("Masukkan angka pertama: ");
                        angka_kedua = baca_angka("Masukkan angka kedua: ");
                        
                        match bagi(angka_pertama, angka_kedua) {
                            Ok(hasil_bagi) => {
                                hasil = hasil_bagi;
                                println!("\nHasil {} / {} = {}", angka_pertama, angka_kedua, hasil);
                            },
                            Err(pesan_error) => {
                                println!("\nError: {}", pesan_error);
                            }
                        }
                        pause();
                    },
                    "5" => {
                        println!("\nTerima kasih telah menggunakan kalkulator ini!");
                        break;
                    },
                    _ => {
                        println!("Pilihan tidak valid. Silakan pilih 1-5.");
                        pause();
                    }
                }
            },
            Err(_) => {
                println!("Gagal membaca input");
                pause();
            }
        }
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    };
}

fn pause() {
    println!("\nTekan Enter untuk melanjutkan...");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
}

fn menu() {
    println!("==== KALKULATOR SEDERHANA ====");
    println!("1. Penjumlahan");
    println!("2. Pengurangan");
    println!("3. Perkalian");
    println!("4. Pembagian");
    println!("5. Keluar");
    println!("==============================");
}

// Fungsi untuk membaca input angka dari pengguna
// Parameter 'pesan' adalah IMMUTABLE - tidak bisa diubah di dalam fungsi
fn baca_angka(pesan: &str) -> f64 {        // pesan: IMMUTABLE reference
    loop {
        let mut input = String::new();    // MUTABLE - untuk menyimpan input sementara
        print!("{} ", pesan);
        io::stdout().flush().unwrap();
        
        match io::stdin().read_line(&mut input) {  // &mut membuat mutable reference ke input
            Ok(_) => {
                match input.trim().parse::<f64>() {
                    Ok(angka) => return angka,  // angka: IMMUTABLE - hanya dibaca
                    Err(_) => println!("Input tidak valid. Masukkan angka yang benar.")
                }
            },
            Err(_) => println!("Gagal membaca input")
        }
    }
}

// Fungsi penjumlahan
pub fn tambah(a: f64, b: f64) -> f64 {
    a + b
}

// Fungsi pengurangan
pub fn kurang(a: f64, b: f64) -> f64 {
    a - b
}

// Fungsi perkalian
pub fn kali(a: f64, b: f64) -> f64 {
    a * b
}

// Fungsi pembagian menggunakan tipe data Result
pub fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Tidak bisa membagi dengan nol".to_string());
    }
    Ok(a / b)
}


// cargo test test_variable -- --nocapture --exact
#[test]
fn test_variable() {
    let name = "Audyari Wiyono";  // IMMUTABLE - tidak bisa diubah
    println!("Hello {}", name);
}

// cargo test test_mutable -- --nocapture --exact
#[test]
fn test_mutable() {
    let mut name = "Audyari Wiyono";  // MUTABLE - bisa diubah
    println!("Hello {}", name);

    name = "Budi Nugraha";  // Nilai diubah karena mutable
    println!("Hello {}", name);
}

// cargo test hello_test -- --nocapture --exact
#[test]
fn hello_test() {
    println!("Hello Test");
}

// cargo test static_typing -- --nocapture --exact
#[test]
fn static_typing() {
    let mut name = "Audyari Wiyono";
    println!("Hello {}", name);
    name = "Budi Nugraha";
    println!("Hello {}", name);
}

// cargo test shadowing -- --nocapture --exact
#[test]
fn shadowing() {
    let name = "Audyari Wiyono";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

// cargo test explicit_type -- --nocapture --exact
#[test]
fn explicit_type() {
let age: i32 = 20;
println! ("{}", age);
}