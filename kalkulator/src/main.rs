use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        clear_screen();
        menu();
        
        let mut pilihan = String::new();
        print!("\nPilih operasi (1-5): ");
        io::stdout().flush().unwrap();
        
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca input");
        
        match pilihan.trim() {
            "1" => println!("Anda memilih Penjumlahan"),
            "2" => println!("Anda memilih Pengurangan"),
            "3" => println!("Anda memilih Perkalian"),
            "4" => println!("Anda memilih Pembagian"),
            "5" => {
                println!("Terima kasih telah menggunakan kalkulator ini!");
                break;
            },
            _ => {
                println!("Pilihan tidak valid. Silakan pilih 1-5.");
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