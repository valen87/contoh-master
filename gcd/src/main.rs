#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

/* Kata kunci fn (melafalkan "fun").
Disini kita mendefinisikan sebuah fungsi yang dinamakan gcd,
dimana mengambil 2 parameter n dan m, yang masing-masing bertipe u64,
sebuah integer yang tidak bertanda tangan. Token -> mendahului tipe
pengembalian: fungsi kami mengembalikan nilai u64. Lekukan empat spasi
adalah gaya Rust standar.

Secara default, setelah variabel diinisialisasi,
nilainya tidak dapat diubah, tetapi menempatkan kata kunci mut
(diucapkan "mute", kependekan dari mutable) sebelum parameter
n dan m memungkinkan badan fungsi kita untuk menetapkannya.
Dalam praktiknya, sebagian besar variabel tidak ditugaskan;
kata kunci mut pada mereka yang melakukannya dapat menjadi
petunjuk yang berguna saat membaca kode.
*/
fn gcd(mut n: u64, mut m: u64) -> u64 {
    /* Tubuh fungsi dimulai dengan panggilan ke assert! macro,
    memverifikasi bahwa tidak ada argumen yang nol. ! karakter menandai
    ini sebagai macro,bukan panggilan fungsi. Seperti assert macro
    di C dan C++, assert! Rust memeriksa apakah argumennya benar, dan
    jika tidak, hentikan program dengan pesan yang berguna termasuk lokasi
    sumber dari pemeriksaan gagal; penghentian mendadak seperti ini
    disebut panic. Tidak seperti C dan C++, dimana pernyataan dapat
    dilewati, Rust selalu memeriksa pernyataan terlepas dari bagaimana
    program dikompilasi. Ada juga debug_assert! macro, yang pernyataan
    dilewati saat program dikompilasi untuk kecepatan.
    */
    assert!(n != 0 && m != 0);

    /* Inti dari fungsi kita adalah perulangan/loop while yang berisi pernyataan
    if dan penugasan. Tidak seperti C dan C++, Rust tidak memerlukan
    tanda kurung di sekitar ekspresi kondisional, tetapi membutuhkan kurung
    kurawal di sekitar pernyataan mereka kontrol.
     */
    /* Pernyataan let mendeklarasikan variabel lokal, seperti t dalam fungsi
    kita. Kita tidak perlu menulis tipe t kita, selama Rust dapat menyimpulkan
    dari cara variabel digunakan. Dalam fungsi kami, satu-satunya tipe yang
    berfungsi untuk t adalah u64, yang cocok dengan m dan n. Rust hanya
    menyimpulkan tipe di dalam badan fungsi: Anda harus menuliskan tipe
    parameter fungsi dan mengembalikan nilai, seperti yang kita lakukan
    sebelumnya. Jika kita ingin mengeja tipe t, kita bisa menulis:
    let t: u64 = m;
     */
    /* Rust memiliki pernyataan kembali, tetapi fungsi gcd tidak membutuhkannya.
    Jika badan fungsi diakhiri dengan ekspresi yang tidak diikuti oleh titik koma,
    itulah nilai kembalian fungsi tersebut. Faktanya, setiap blok yang dikelilingi
    oleh kurung kurawal dapat berfungsi sebagai ekspresi. Misalnya, ini adalah
    ekspresi yang mencetak pesan dan kemudian menghasilkan x.cos()
    sebagai nilainya:
    {
        println!("evaluating cos x");
    }
     */
    /* Biasanya di Rust menggunakan formulir ini untuk menetapkan nilai fungsi saat
    kontrol "jatuh dari akhir" fungsi, dan menggunakan pernyataan pengembalian hanya
    untuk pengembalian awal yang eksplisit dari tengah-tengah suatu fungsi.
     */
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/* Rust memiliki dukungan sederhana untuk pengujian yang dibangun ke dalam
bahasa. Untuk menguji fungsi gcd kami, kita dapat menambahkan kode ini di
akhir src/main.rs
 */
/* Di sini kita mendefinisikan sebuah fungsi bernama test_gcd, yang memanggil
gcd dan memeriksa apakah itu mengembalikan nilai yang benar. #[test] di atas
definisi menandai test_gcd sebagai fungsi pengujian, untuk dilewati dalam
kompilasi normal, tetapi disertakan dan dipanggil secara otomatis jika
kita menjalankan program kita dengan perintah cargo test. Kami dapat memiliki
fungsi pengujian yang tersebar di seluruh pohon sumber kami, ditempatkan di
sebelah kode yang mereka jalankan, dan pengujian kargo akan secara otomatis
mengumpulkannya dan menjalankan semuanya.
 */
/* Penanda #[test] adalah contoh atribut. Atribut adalah sistem terbuka untuk
 menandai fungsi dan deklarasi lain dengan informasi tambahan, seperti atribut
 di C++ dan C#, atau anotasi di Java. Mereka digunakan untuk mengontrol
 peringatan kompiler dan pemeriksaan gaya kode, menyertakan kode secara
 kondisional (seperti #ifdef di C dan C++), memberi tahu Rust cara berinteraksi
dengan kode yang ditulis dalam bahasa lain, dan seterusnya. Kita akan melihat
lebih banyak contoh atribut seiring berjalannya waktu.
 */
/* Dengan definisi gcd dan test_gcd kami ditambahkan ke paket hello yang kami
buat di awal bab, dan direktori kami saat ini di suatu tempat di dalam
subpohon paket, kami dapat menjalankan tes sebagai berikut:
$ cargo test
   Compiling hello v0.1.0 (/home/jimb/rust/hello)
    Finished test [unoptimized + debuginfo] target(s) in 0.35s
     Running /home/jimb/rust/hello/target/debug/deps/hello-2375a82d9e9673d7
running 1 test
test test_gcd ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
 */
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

/* Agar program kami mengambil serangkaian angka sebagai argumen baris
perintah dan mencetak pembagi persekutuan terbesarnya, kami dapat mengganti
fungsi utama di src/main.rs dengan yang berikut:
 */
/* Deklarasi penggunaan paling cocok membawa trait perpustakaan standar
FromStr ke dalam cakupan. Trait adalah kumpulan metode yang diterapkan oleh tipe.
Setiap jenis yang mengimplemetasikan sifat FromStr memiliki metode from_str yang
mencoba mengurai nilai jenis tersebut dari sebuah string. Tipe u64
mengimplementasikan FromStr, dan kita akan memanggil u64::from_str untuk mengurai
argumen baris perintah kita. Meskipun kami tidak pernah menggunakan nama FromStr
di tempat lain dalam program, suatu trait harus ada dalam cakupan untuk
menggunakan metodenya.
 */
/* Deklarasi penggunaan kedua membawa modul std::env, yang menyediakan beberapa
fungsi dan tipe yang berguna untuk berinteraksi dengan lingkungan eksekusi,
termasuk args, yang memberi kita akses ke argumen baris perintah program.
 */
use std::env;
use std::str::FromStr;

/* Fungsi utama kami tidak mengembalikan nilai, jadi kami dapat dengan mudah
menghilangkan -> jenis yang biasanya mengikuti daftar parameter.
 */
fn main() {
    /* Kami mendeklarasikan nomor variabel lokal yang dapat diubah dan
    menginisialisasinya ke vektor kosong. Vec adalah tipe vektor Rust yang dapat
    ditumbuhkan, analog dengan std::vector C++, daftar Python, atau larik JavaScript.
    Meskipun vektor dirancang untuk tumbuh dan menyusut secara dinamis, kita masih
    harus menandai variabel mut agar Rust memungkinkan kita memasukkan angka
    ke ujungnya.
    */
    /* Jenis angkanya adalah Vec<u64>, sebuah vektor dengan nilai u64, tetapi seperti
    sebelumnya, kita tidak perlu menuliskannya. Rust akan menyimpulkannya untuk kita,
    sebagian karena apa yang kita tekan ke vektor adalah nilai u64, tetapi juga karena
    kita meneruskan elemen vektor ke gcd, yang hanya menerima nilai u64.
    */
    let mut numbers = Vec::new();

    /* Di sini kita menggunakan for loop untuk memproses argumen baris perintah kita,
    menyetel variabel arg ke setiap argumen secara bergantian dan mengevaluasi badan loop.
     */
    /* Fungsi args modul std::env mengembalikan iterator, nilai yang menghasilkan setiap
    argumen sesuai permintaan, dan menunjukkan kapan kita selesai. Iterator ada di mana-mana
    di Rust; perpustakaan standar mencakup iterator lain yang menghasilkan elemen vektor,
    baris file, pesan yang diterima pada saluran komunikasi, dan hampir
    hal lain yang masuk akal untuk diulang. Iterator Rust sangat efisien: kompiler biasanya
    dapat menerjemahkannya ke dalam kode yang sama dengan loop tulisan tangan.
    */
    /* Di luar penggunaannya dengan for loop, iterator menyertakan banyak pilihan metode
    yang dapat Anda gunakan secara langsung. Misalnya, nilai pertama yang dihasilkan
    oleh iterator yang dikembalikan oleh args selalu merupakan nama program yang sedang
    dijalankan. Kami ingin melewati itu, jadi kami memanggil metode lewati iterator untuk
    menghasilkan iterator baru yang menghilangkan nilai pertama itu.
    */
    for arg in env::args().skip(1) {
        /* Di sini kita memanggil u64::from_str untuk mencoba mengurai argumen baris perintah kita
        sebagai integer 64-bit yang tidak ditandatangani. Alih-alih metode yang kami gunakan pada
        beberapa nilai u64 yang kami miliki, u64::from_str adalah fungsi yang terkait dengan tipe u64,
        mirip dengan metode statis di C++ atau Java. Fungsi from_str tidak mengembalikan u64
        secara langsung, melainkan nilai Hasil yang menunjukkan apakah penguraian berhasil atau gagal.
        Nilai Hasil adalah salah satu dari dua varian:
            • Nilai tertulis Ok(v), menunjukkan bahwa parse berhasil dan v adalah nilai yang dihasilkan
            • Nilai yang ditulis Err(e), menunjukkan bahwa parse gagal dan e adalah nilai kesalahan yang
            menjelaskan alasannya
        */
        /* Fungsi yang melakukan apa pun yang mungkin gagal, seperti melakukan input atau output atau
        berinteraksi dengan sistem operasi, dapat mengembalikan tipe Hasil yang varian Oknya
        membawa hasil yang berhasil—jumlah byte yang ditransfer, file dibuka, dan seterusnya— dan
        yang Err varian membawa kode kesalahan yang menunjukkan apa yang salah. Tidak seperti
        kebanyakan bahasa modern, Rust tidak memiliki pengecualian: semua kesalahan ditangani
        menggunakan Result atau panic
        */
        /* Kami menggunakan metode harapan Hasil untuk memeriksa keberhasilan penguraian kami.
        Jika hasilnya adalah Err(e), harapkan mencetak pesan yang menyertakan deskripsi e dan
        segera keluar dari program. Namun, jika hasilnya adalah Ok(v), harapkan hanya
        mengembalikan v itu sendiri, yang akhirnya dapat kita tekan ke akhir vektor angka kita.
        */
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    /* Tidak ada pembagi persekutuan terbesar dari kumpulan angka kosong, jadi kami memeriksa
    bahwa vektor kami memiliki setidaknya satu elemen dan keluar dari program dengan kesalahan
    jika tidak Kami menggunakan eprintln! makro untuk menulis pesan kesalahan kami ke aliran
    keluaran kesalahan standar
    */
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    /* Loop ini menggunakan d sebagai nilai berjalannya, memperbaruinya agar tetap menjadi
    pembagi umum terbesar dari semua angka yang telah kita proses sejauh ini.
    Seperti sebelumnya, kita harus menandai d sebagai bisa berubah sehingga kita dapat
    menetapkannya dalam loop.
    */
    /* Perulangan for memiliki dua bit yang mengejutkan. Pertama, kita menulis untuk m di
    &numbers[1..]; untuk apa & operatornya? Kedua, kita menulis gcd(d, *m); untuk apa * dalam *m?
    Kedua detail ini saling melengkapi.
    */
    /* Hingga saat ini, kode kami hanya beroperasi pada nilai sederhana seperti bilangan bulat
    yang sesuai dengan blok memori berukuran tetap. Tapi sekarang kita akan mengulangi sebuah
    vektor, yang bisa berukuran berapa pun—mungkin sangat besar. Rust berhati-hati saat menangani
    nilai-nilai seperti itu: ia ingin membiarkan programmer mengendalikan konsumsi memori,
    memperjelas berapa lama setiap nilai hidup, sambil tetap memastikan memori segera dibebaskan
    saat tidak lagi dibutuhkan.
    */
    /* Jadi ketika kami mengulangi, kami ingin memberi tahu Rust bahwa kepemilikan vektor harus
    tetap dengan angka; kita hanya meminjam elemennya untuk loop. Operator & dalam &numbers[1..]
    meminjam referensi ke elemen vektor dari yang kedua dan seterusnya. Perulangan for mengulangi
    elemen yang direferensikan, membiarkan m meminjam setiap elemen secara berurutan. Operator *
    dalam *m mendereferensi m, menghasilkan nilai yang dirujuknya; ini adalah u64 berikutnya
    yang ingin kami berikan ke gcd. Akhirnya, karena angka memiliki vektor, Rust secara otomatis
    membebaskannya ketika angka keluar dari ruang lingkup di akhir utama.
    */
    /* Aturan Rust untuk kepemilikan dan referensi adalah kunci untuk manajemen memori Rust dan
    konkurensi yang aman. Anda harus terbiasa dengan aturan tersebut agar nyaman di Rust, tetapi
    untuk tur pendahuluan ini, yang perlu Anda ketahui hanyalah bahwa &x meminjam referensi ke x,
    dan *r adalah nilai yang dirujuk oleh referensi r.

    Melanjutkan perjalanan kami melalui program:
    */
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    /* Setelah mengulangi elemen angka, program mencetak hasilnya ke aliran keluaran standar.
    CPrint! makro mengambil string templat, mengganti versi yang diformat dari argumen yang
    tersisa untuk formulir {...} seperti yang muncul di string templat, dan menulis hasilnya
    ke aliran keluaran standar.
    */
    /* Tidak seperti C dan C++, yang mengharuskan main untuk mengembalikan nol jika program
    selesai dengan sukses, atau status keluar bukan nol jika ada yang tidak beres,
    Rust berasumsi bahwa jika main kembali, program selesai dengan sukses. Hanya dengan
    memanggil fungsi secara eksplisit seperti expect atau std::process::exit kita dapat
    menyebabkan program dihentikan dengan kode status kesalahan.
    */
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

/* Perintah cargo run memungkinkan kita untuk meneruskan argumen ke program kita,
sehingga kita dapat mencoba penanganan baris perintah kita:

$ cargo run 42 56
Compiling hello v0.1.0 (/home/jimb/rust/hello)
Finished dev [unoptimized + debuginfo] target(s) in 0.22s
Running `/home/jimb/rust/hello/target/debug/hello 42 56`
The greatest common divisor of [42, 56] is 14
$cargo run 799459 28823 27347
Finished dev [unoptimized + debuginfo] target(s) in 0.22s
Running `/home/jimb/rust/hello/target/debug/hello 799459 28823 27347`
The greatest common divisor of [799459, 28823, 27347] is 41
$cargo run 83
Finished dev [unoptimized + debuginfo] target(s) in 0.22s
Running `/home/jimb/rust/hello/target/debug/hello 83`
The greatest common divisor of [83] is 83
$cargo run
Finished dev [unoptimized + debuginfo] target(s) in 0.22s
Running `/home/jimb/rust/hello/target/debug/hello`
Usage: gcd NUMBER ...
*/
/* Kami telah menggunakan beberapa fitur dari pustaka standar Rust di bagian ini.
Jika anda penasaran dengan apa lagi yang tersedia, kami sangat menyarankan anda
untuk mencoba dokumentasi online Rust. Ini memiliki fitur pencarian langsung
yang memudahkan eksplorasi dan bahkan menyertakan tautan ke kode sumber.
Perintah rustup secara otomatis menginstal salinan di komputer anda saat
menginstal Rust itu sendiri. Anda dapat melihat dokumentasi perpustakaan
standar di situs web Rust, atau di browser anda dengan perintah:

$rustup doc --std
*/
