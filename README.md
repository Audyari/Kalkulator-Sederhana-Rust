# Kalkulator Sederhana dengan Rust

Sebuah kalkulator sederhana yang dibangun menggunakan bahasa pemrograman Rust. Aplikasi ini beroperasi melalui command line dan mendukung operasi matematika dasar seperti penjumlahan, pengurangan, perkalian, dan pembagian.

## Fitur

- [x] Menu interaktif
- [x] Operasi dasar (+, -, *, /)
- [x] Antarmuka baris perintah yang user-friendly
- [x] Validasi input
- [ ] Riwayat perhitungan
- [ ] Dukungan untuk bilangan desimal
- [ ] Mode kalkulator ilmiah

## Variabel yang Dibutuhkan

### Variabel Input
- `angka_pertama: f64` - Menyimpan angka pertama untuk perhitungan
- `angka_kedua: f64` - Menyimpan angka kedua untuk perhitungan
- `pilihan: String` - Menyimpan pilihan operasi dari pengguna (1-5)

### Variabel Output
- `hasil: f64` - Menyimpan hasil perhitungan
- `pesan_error: String` - Menyimpan pesan error jika terjadi kesalahan

### Variabel Kontrol
- `lanjut: bool` - Mengontrol perulangan program
- `input_valid: bool` - Memvalidasi input pengguna

## Daftar Tugas

### Task 1: Siapkan Lingkungan Pengembangan
- [x] Install Rust
  - [x] Kunjungi [rust-lang.org](https://www.rust-lang.org/)
  - [x] Ikuti petunjuk instalasi untuk sistem operasi kamu
- [x] Buat proyek baru
  - [x] Jalankan perintah `cargo new kalkulator`
  - [x] Masuk ke direktori proyek dengan `cd kalkulator`

### Task 2: Build dan Run
- [x] `cargo run`
- [x] `cargo build --release`

### Task 3: Unit-Testing
- [x] Tambahkan unit testing
- [x] Tambahkan unit testing untuk operasi matematika dasar
- [x] Tambahkan unit testing untuk operasi matematika kompleks

### Task 4: Variabel yang Dibutuhkan
- [x] Tambahkan variabel yang dibutuhkan

### Task 5: Variabel Mutable dan Immutable

  #### Task 5.1 Variabel Mutable (Bisa Diubah)

  * Di fungsi `main()`: `angka_pertama`, `angka_kedua`, `hasil`, `pesan_error`, `lanjut`, `input_valid`, `pilihan`
  * Di fungsi `baca_angka()`: `input`
  * Di fungsi `pause()`: `temp`
  * Di test function: `name` dalam `test_mutable()`

  #### Task 5.2 Variabel Immutable (Tidak Bisa Diubah)

  * Di fungsi `baca_angka()`: `pesan`, `angka`
  * Di test function: `name` dalam `test_variable()`

### Task 6: Gunakan Static Typing
- [x] Tambahkan static typing

### Task 7: Jangan ada Shadowing
- [x] Jangan ada shadowing
- [x] Tambahkan unit test anti-shadowing
- [x] Gunakan nama variabel yang deskriptif dan unik
- [x] Hindari menggunakan nama yang sama di scope yang sama
- [x] Aktifkan peringatan shadowing di Rust:

### Task 8: Data Type
- [x] Tipe Data Dasar (Primitif)
    - f64 (64-bit floating point)
    - String
    - bool
- [x] Tipe Data Komposit  
    - Result<T, E> contoh di aplikasi : pub fn bagi()

- [x] Tipe Data Referensi
    - &str (baca_angka)

- [x] Tipe Data Unit
    - () fn clear_screen()

- [x] Tipe Data dari Standard Library
    - std::io::Result
    - Digunakan untuk menangani operasi I/O
 
### Task 9: Input Pengguna
- [x] Tambahkan input untuk memilih operasi
  - [x] Gunakan `std::io::stdin()` untuk membaca input
- [ ] Tambahkan input untuk dua angka
- [ ] Pastikan untuk mengonversi input ke tipe data yang tepat

### Task 10: Implementasi Operasi Dasar
- [ ] Tambahkan fungsi untuk penjumlahan
- [ ] Tambahkan fungsi untuk pengurangan
- [ ] Tambahkan fungsi untuk perkalian
- [ ] Tambahkan fungsi untuk pembagian
  - [ ] Pastikan untuk menangani pembagian dengan nol

### Task 11: Lakukan Perhitungan
- [ ] Tambahkan logika untuk memilih operasi berdasarkan input pengguna
- [ ] Panggil fungsi operasi yang sesuai

### Task 12: Tampilkan Hasil
- [ ] Tampilkan hasil perhitungan ke pengguna
- [ ] Tanyakan kepada pengguna jika ingin melakukan perhitungan lagi

### Task 13: Uji Aplikasi
- [ ] Jalankan aplikasi dengan `cargo run`
- [ ] Uji semua operasi dengan berbagai input
- [ ] Perbaiki bug yang ditemukan

### Task 14: Dokumentasi
- [ ] Tambahkan komentar di dalam kode untuk penjelasan
- [x] Buat README.md dengan penjelasan tentang cara menggunakan kalkulator

### Task 15: (Opsional) Tambahkan Fitur Tambahan
- [ ] Tambahkan kemampuan untuk mendukung operasi lebih lanjut (mis. persen, eksponen)
- [ ] Tambahkan antarmuka pengguna grafis (GUI) menggunakan pustaka seperti `gtk-rs` atau `druid`