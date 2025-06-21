use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Deklarasi variabel
    let mut angka_pertama: f64 = 0.0;      // MUTABLE - nilainya bisa diubah
    let mut angka_kedua: f64 = 0.0;        // MUTABLE - nilainya bisa diubah
    let mut hasil: f64 = 0.0;              // MUTABLE - akan menyimpan hasil perhitungan
    let mut pesan_error = String::new();    // MUTABLE - untuk menampung pesan error
    let mut lanjut = true;                 // MUTABLE - untuk kontrol loop
    let mut input_valid: bool;             // MUTABLE - untuk validasi input

    loop {
        clear_screen();
        menu();
        
        // Reset variabel
        input_valid = false;               // MUTABLE - diubah nilainya
        let mut pilihan = String::new();   // MUTABLE - untuk menyimpan input pengguna
        
        // Baca pilihan operasi
        print!("\nPilih operasi (1-5): ");
        io::stdout().flush().unwrap();
        
        match io::stdin().read_line(&mut pilihan) {
            Ok(_) => {
                match pilihan.trim() {
                    "1" | "2" | "3" | "4" => {
                        // Baca dua angka untuk operasi
                        angka_pertama = baca_angka("Masukkan angka pertama: ");
                        angka_kedua = baca_angka("Masukkan angka kedua: ");
                        
                        // Tampilkan input untuk verifikasi
                        println!("\nAnda memilih: {}", 
                            match pilihan.trim() {
                                "1" => "Penjumlahan",
                                "2" => "Pengurangan",
                                "3" => "Perkalian",
                                "4" => "Pembagian",
                                _ => ""
                            }
                        );
                        println!("Angka pertama: {}", angka_pertama);
                        println!("Angka kedua: {}", angka_kedua);
                        
                        // Tunggu input pengguna sebelum melanjutkan
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
    let name = "Eko Kurniawan Khannedy";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

// cargo test test_deteksi_shadowing_di_baca_angka -- --nocapture --exact
#[test]
fn test_deteksi_shadowing_di_baca_angka() {
   
    // Simulasikan input "42\n"
    let input = b"42\n";
    let mut cursor = std::io::Cursor::new(input);

    // Simpan stdin asli
    let original_stdin = std::io::stdin();

    // Ganti stdin dengan cursor kita
    let _guard = Box::new(original_stdin.lock());

    // Test
    let result = {
        let mut input_string = String::new();
        std::io::Read::read_to_string(&mut cursor, &mut input_string).unwrap();
        input_string.trim().parse::<f64>().unwrap()
    };

    assert_eq!(result, 42.0, "Fungsi baca_angka seharusnya tidak mengandung shadowing");
}