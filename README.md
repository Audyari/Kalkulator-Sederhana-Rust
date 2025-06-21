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

### Task 4: Input Pengguna
- [x] Tambahkan input untuk memilih operasi
  - [x] Gunakan `std::io::stdin()` untuk membaca input
- [ ] Tambahkan input untuk dua angka
- [ ] Pastikan untuk mengonversi input ke tipe data yang tepat

### Task 5: Implementasi Operasi Dasar
- [ ] Tambahkan fungsi untuk penjumlahan
- [ ] Tambahkan fungsi untuk pengurangan
- [ ] Tambahkan fungsi untuk perkalian
- [ ] Tambahkan fungsi untuk pembagian
  - [ ] Pastikan untuk menangani pembagian dengan nol

### Task 6: Lakukan Perhitungan
- [ ] Tambahkan logika untuk memilih operasi berdasarkan input pengguna
- [ ] Panggil fungsi operasi yang sesuai

### Task 7: Tampilkan Hasil
- [ ] Tampilkan hasil perhitungan ke pengguna
- [ ] Tanyakan kepada pengguna jika ingin melakukan perhitungan lagi

### Task 8: Uji Aplikasi
- [ ] Jalankan aplikasi dengan `cargo run`
- [ ] Uji semua operasi dengan berbagai input
- [ ] Perbaiki bug yang ditemukan

### Task 9: Dokumentasi
- [ ] Tambahkan komentar di dalam kode untuk penjelasan
- [x] Buat README.md dengan penjelasan tentang cara menggunakan kalkulator

### Task 10: (Opsional) Tambahkan Fitur Tambahan
- [ ] Tambahkan kemampuan untuk mendukung operasi lebih lanjut (mis. persen, eksponen)
- [ ] Tambahkan antarmuka pengguna grafis (GUI) menggunakan pustaka seperti `gtk-rs` atau `druid`