# Proyek: Kalkulator Sederhana dengan Rust

## Task 1: Siapkan Lingkungan Pengembangan
- [ ] Install Rust
  - [ ] Kunjungi [rust-lang.org](https://www.rust-lang.org/)
  - [ ] Ikuti petunjuk instalasi untuk sistem operasi kamu
- [ ] Buat proyek baru
  - [ ] Jalankan perintah `cargo new kalkulator`
  - [ ] Masuk ke direktori proyek dengan `cd kalkulator`

## Task 2: Struktur Program
- [ ] Buka file `src/main.rs`
- [ ] Tambahkan fungsi `main`
  - [ ] Buat fungsi untuk mencetak menu

## Task 3: Input Pengguna
- [ ] Tambahkan input untuk memilih operasi
  - [ ] Gunakan `std::io::stdin()` untuk membaca input
- [ ] Tambahkan input untuk dua angka
  - [ ] Pastikan untuk mengonversi input ke tipe data yang tepat

## Task 4: Implementasi Operasi Dasar
- [ ] Tambahkan fungsi untuk penjumlahan
- [ ] Tambahkan fungsi untuk pengurangan
- [ ] Tambahkan fungsi untuk perkalian
- [ ] Tambahkan fungsi untuk pembagian
  - [ ] Pastikan untuk menangani pembagian dengan nol

## Task 5: Lakukan Perhitungan
- [ ] Tambahkan logika untuk memilih operasi berdasarkan input pengguna
- [ ] Panggil fungsi operasi yang sesuai

## Task 6: Tampilkan Hasil
- [ ] Tampilkan hasil perhitungan ke pengguna
- [ ] Tanyakan kepada pengguna jika ingin melakukan perhitungan lagi

## Task 7: Uji Aplikasi
- [ ] Jalankan aplikasi dengan `cargo run`
- [ ] Uji semua operasi dengan berbagai input
- [ ] Perbaiki bug yang ditemukan

## Task 8: Dokumentasi
- [ ] Tambahkan komentar di dalam kode untuk penjelasan
- [ ] Buat README.md dengan penjelasan tentang cara menggunakan kalkulator

## Task 9: (Opsional) Tambahkan Fitur Tambahan
- [ ] Tambahkan kemampuan untuk mendukung operasi lebih lanjut (mis. persen, eksponen)
- [ ] Tambahkan antarmuka pengguna grafis (GUI) menggunakan pustaka seperti `gtk-rs` atau `druid`