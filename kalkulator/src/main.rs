use std::io::{self, Write};
use std::process::Command;

// Konstanta untuk pesan error
const ERR_DIV_ZERO: &str = "Error: Tidak bisa membagi dengan nol";
const ERR_INVALID_INPUT: &str = "Error: Input tidak valid";

// Batas input
const MAX_INPUT: f64 = 1_000_000.0;
const MIN_INPUT: f64 = -1_000_000.0;



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
        print!("\nPilih operasi (1-6): ");
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
                    // Minta input pengguna terlebih dahulu
                    angka_pertama = baca_angka("Masukkan angka pertama: ");
                    angka_kedua = baca_angka("Masukkan angka kedua: ");
    
                    // Menggunakan tuple untuk menampung hasil
                    let (tambah, kurang, kali, bagi, status) = hitung_semua(angka_pertama, angka_kedua);

                    println!("\n=== Hasil Semua Operasi ===");
                    println!("{} + {} = {}", angka_pertama, angka_kedua, tambah);
                    println!("{} - {} = {}", angka_pertama, angka_kedua, kurang);
                    println!("{} * {} = {}", angka_pertama, angka_kedua, kali);
    
                    if angka_kedua != 0.0 {
                        println!("{} / {} = {}", angka_pertama, angka_kedua, bagi);
                    } else {
                        println!("Pembagian dengan nol tidak diizinkan");
                    }
    
                    println!("Status: {}", status);
                    pause();
                },
                   "6" => {
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

// Fungsi untuk menampilkan menu
fn menu() {
    println!("==== KALKULATOR SEDERHANA ====");
    println!("1. Penjumlahan");
    println!("2. Pengurangan");
    println!("3. Perkalian");
    println!("4. Pembagian");
    println!("5. Semua Operasi (Menggunakan Tuple)");
    println!("6. Keluar");
    println!("==============================");
}

// Fungsi untuk membaca input angka dari pengguna
fn baca_angka(pesan: &str) -> f64 {
    loop {
        let mut input = String::new();
        print!("{} ", pesan);
        io::stdout().flush().unwrap();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<f64>() {
                    Ok(angka) if angka >= MIN_INPUT && angka <= MAX_INPUT => return angka,
                    Ok(_) => println!("Angka harus antara {} dan {}", MIN_INPUT, MAX_INPUT),
                    Err(_) => println!("{}", ERR_INVALID_INPUT)
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
        //return Err("Tidak bisa membagi dengan nol".to_string());
        return Err(ERR_DIV_ZERO.to_string());
    }
    Ok(a / b)
}

// Fungsi yang mengembalikan tuple (f64, f64, &'static str)
fn hitung_semua(a: f64, b: f64) -> (f64, f64, f64, f64, &'static str) {
    let tambah = a + b;
    let kurang = a - b;
    let kali = a * b;
    
    let (bagi, status) = if b != 0.0 {
        (a / b, "Berhasil")
    } else {
        (f64::NAN, ERR_DIV_ZERO)
    };
    
    (tambah, kurang, kali, bagi, status)
}

// cargo test test_tuple -- --nocapture --exact
#[test]
fn test_tuple() {
    let (tambah, kurang, kali, bagi, status) = hitung_semua(10.0, 2.0);
    println!("Hasil penjumlahan: {}", tambah);
    println!("Hasil pengurangan: {}", kurang);
    println!("Hasil perkalian: {}", kali);
    println!("Hasil pembagian: {}", bagi);
    println!("Status: {}", status);
    
    // Contoh akses elemen tuple dengan indeks
    let hasil = hitung_semua(10.0, 2.0);
    println!("Hasil perkalian (menggunakan indeks): {}", hasil.2);
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

// cargo test number -- --nocapture --exact
#[test]
fn number() {
    let a: i32 = 10;
    println!("Tetapkan number: {}", a);

    let b: f64 = 10.5;
    println!("Tetapkan float: {}", b);
}

// cargo test number_conversion -- --nocapture --exact
#[test]
fn number_conversion() {
    let a: i8 = 16;
    println!("{}" , a);
    let b: i16 = a as i16;
    println!("{}", b);
    let c: i32 = a as i32;
    println!("{}", c);
}

// cargo test numeric_operator_of -- --nocapture --exact
#[test]
fn numeric_operator_of() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
}

// cargo test augmented_assignment -- --nocapture --exact
#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{} ", a);

    a += 10;
    println!("{} ", a);

    a -= 10;
    println!("{} ", a);
}

// cargo test boolean_of -- --nocapture --exact
#[test]
fn boolean_of() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

// cargo test comparison -- --nocapture --exact
#[test]
fn comparison() {
    let result: bool = 10 > 20;
    println!("{}", result);
}

// cargo test boolean_operator -- --nocapture --exact
#[test]
fn boolean_operator() {
    let absen = 76;
    let nilai_akhir = 860;
    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 75;
    let lulus_final = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus_final);
}

// cargo test test_aritmatika_dasar -- --nocapture --exact
#[test]
fn test_aritmatika_dasar() {
    // Test penjumlahan
    assert_eq!(tambah(5.0, 3.0), 8.0);
    assert_eq!(tambah(-5.0, 3.0), -2.0);
    assert_eq!(tambah(0.0, 0.0), 0.0);
    
    // Test pengurangan
    assert_eq!(kurang(5.0, 3.0), 2.0);
    assert_eq!(kurang(3.0, 5.0), -2.0);
    
    // Test perkalian
    assert_eq!(kali(5.0, 3.0), 15.0);
    assert_eq!(kali(5.0, 0.0), 0.0);
    
    // Test pembagian
    assert!(bagi(6.0, 3.0).is_ok_and(|x| (x - 2.0).abs() < f64::EPSILON));
    assert!(bagi(5.0, 0.0).is_err());
    assert!(bagi(0.0, 5.0).is_ok_and(|x| (x - 0.0).abs() < f64::EPSILON));
}

// cargo test test_hitung_semua -- --nocapture --exact
#[test]
fn test_hitung_semua() {
    // Test kasus normal
    let (tambah, kurang, kali, bagi, status) = hitung_semua(10.0, 2.0);
    assert!((tambah - 12.0).abs() < f64::EPSILON);
    assert!((kurang - 8.0).abs() < f64::EPSILON);
    assert!((kali - 20.0).abs() < f64::EPSILON);
    assert!((bagi - 5.0).abs() < f64::EPSILON);
    assert_eq!(status, "Berhasil");
    
    // Test pembagian dengan nol
    let (_, _, _, bagi, status) = hitung_semua(10.0, 0.0);
    assert!(bagi.is_nan());
    assert_eq!(status, "Error: Pembagian nol");
    
    // Test angka negatif
    let (tambah, kurang, kali, _bagi, _) = hitung_semua(-5.0, 3.0);
    assert!((tambah - (-2.0)).abs() < f64::EPSILON);
    assert!((kurang - (-8.0)).abs() < f64::EPSILON);
    assert!((kali - (-15.0)).abs() < f64::EPSILON);
}

// cargo test char_type -- --nocapture --exact
#[test]
fn char_type() {
    let char1: char = 'a';
    let char2: char = 'b';
    println!("{} {}", char1, char2);
}

// cargo test tuple -- --nocapture --exact
#[test]
fn tuple() {
    let data: (i32, f64, bool) = (18, 10.5, true);
    println!("{:?}", data);
}

// cargo test tuple_access -- --nocapture --exact
#[test]
fn tuple_access() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);
}

// cargo test tuple_dereference -- --nocapture --exact
#[test]
fn tuple_dereference() {
    let data: (i32, f64, bool) = (16, 10.5, true);
    println!("{:?}", data);
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}

// cargo test tuple_mutability -- --nocapture --exact
#[test]
fn mutable_tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;
    println!("{:?}", data);
}

// cargo test unit_type -- --nocapture --exact
#[test]
fn test_unit_type() {
    let _unit: () = ();  // Inisialisasi tipe unit
    let result = println!("Ini adalah tipe unit");
    assert_eq!(result, ());  // println! mengembalikan ()
    
    let fungsi_kosong = || {};
    assert_eq!(fungsi_kosong(), ());
}

// cargo test array -- --nocapture --exact
#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
}

// cargo test array_access -- --nocapture --exact
#[test]
fn array_access() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);
}

// cargo test array_mutability -- --nocapture --exact
#[test]
fn array_mutability() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 16;
    array[1] = 20;
    println!("{:?}", array);
}

// cargo test array_length -- --nocapture --exact
#[test]
fn array_length() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    let length: usize = array.len();
    println!("Panjang array: {}", length);
}

// cargo test two_dimensional_array -- --nocapture --exact
#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 3] = [
        [32, 41],
        [1, 6],
        [11, 16],
    ];

    println!("{:?}", matrix);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[2][1]);
    println!("{}", matrix[1][1]);
}

const MAXIMUM: i32 = 160;

// cargo test constant -- --nocapture --exact
#[test]
fn constant(){
const MINIMUM: i32 = 0;
println!("{} {}", MINIMUM, MAXIMUM);
}