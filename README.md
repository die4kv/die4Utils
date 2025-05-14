<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
</head>
<body>
  <h3>die4Utils</h3>

  <p2><strong>die4Utils</strong> adalah kumpulan utilitas baris perintah ala Unix, ditulis dengan Rust 2024 edition.<br>
  Mencakup: <code>echo</code>, <code>cat</code>, <code>ls</code>, <code>find</code>, dan <code>grep</code>.</p2>

  <h3>Fitur</h3>
  <ul>
    <li><strong>echo</strong>: Menampilkan teks ke output</li>
    <li><strong>cat</strong>: Menampilkan isi file dengan opsi penomoran baris</li>
    <li><strong>ls</strong>: Melihat isi direktori dengan dukungan file tersembunyi</li>
    <li><strong>find</strong>: Mencari file berdasarkan nama</li>
    <li><strong>grep</strong>: Mencari pola teks dalam file (rekursif dan case-insensitive opsional)</li>
    <li><strong>Penanganan error yang baik</strong> (tanpa <code>.unwrap()</code> atau <code>.expect()</code>)</li>
  </ul>

  <h3>Persyaratan: </h3>
  <ul>
    <li><a href="https://www.rust-lang.org/learn/get-started" target="_blank">Rust dan cargo melalui rustup</a></li>
  </ul>

  <h3>Langkah Instalasi: </h3>
  <pre><code>git clone https://github.com/die4kv/die4Utils.git
cd die4Utils
chmod +x ./install.sh
./install.sh</code></pre>

  <h3>Opsional Install: </h3>
  <pre><code>
    Usage: ./install.sh [OPTIONS]    
     Options:
        --local           Install to $HOME/.cargo/bin
        --global          Install to /usr/local/bin
        --shell [bash|zsh|fish]  Set PATH in shell config (only for --local)
        -h, --help        Show this help message
  </code></pre>

  <hr>

  <h3>Cara Penggunaan</h3>
  <p2>Jalankan:</p2>
  <pre><code>die4Utils --help</code></pre>

  <hr>

  <h3>Testing</h3>
  <p2>Untuk menjalankan pengujian:</p2>
  <pre><code>cargo test</code></pre>

  <hr>

  <h3>Laporan Masalah</h3>
  <p2>Jika mengalami kendala saat instalasi atau penggunaan, silakan laporkan ke:<br>
  <a href="https://github.com/die4kv/die4Utils/issues" target="_blank">https://github.com/die4kv/die4Utils/issues</a></p2>
</body>
</html>
