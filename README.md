## die4Utils

**die4Utils** adalah kumpulan utilitas baris perintah ala Unix, ditulis dengan Rust 2024 edition.  
Mencakup: `echo`, `cat`, `ls`, `find`, dan `grep`.  


### Fitur  
  
- **echo**: Menampilkan teks ke output  
- **cat**: Menampilkan isi file dengan opsi nomor baris  
- **ls**: Melihat isi direktori dengan opsi file tersembunyi  
- **find**: Mencari file berdasarkan nama  
- **grep**: Mencari pola teks dalam file (rekursif dan case-insensitive opsional)  
- **Output berwarna** dan penanganan error yang baik  
  
### Instalasi  
**1. Persyaratan**

  • [rustup](https://www.rust-lang.org/learn/get-started)  
    
**2. Jalankan**

```bash  
git clone https://github.com/die4kv/die4Utils.git  
cd die4Utils 
cargo build --release  
```

**3. Harus**  

**• local**  
  
- contoh untuk menggunakannya hanya di ~/Documents:   
```
cp -r ./target/release/die4Utils ~/Documents  
  
```
- gunakan ./die4Utils untuk menjalankannya

**• global**

- pastikan  ~/.local/bin sudah dalam PATH:
```
cp -r ./target/release/die4Utils ~/.local/bin

```

**• Cara Penggunaan**  
    
  
  **Use**: die4Utils --help  
  
     
jika ada masalah saat instalasi report ke [sini](https://github.com/die4kv/die4Utils/issues)  
  
  
  
### Testing

```
cargo test
  
```
