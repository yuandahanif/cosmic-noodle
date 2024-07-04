# laporan pengjian

Proof of concept sistem pemilahan sampah menggunakan model YOLOv8 dan dataset TACO. Tujuan dari proyek ini adalah sebagai pembuktian konsep bahwa model YOLOv8 dapat digunakan untuk membantu memilah sampah. [Model YOLO](https://github.com/ultralytics/ultralytics) adalah model yang digunakan untuk mendeteksi objek dalam gambar. [Dataset TACO](http://tacodataset.org) atau Trash Annotations in Context adalah dataset yang berisi gambar sampah yang telah diberi anotasi. Dataset ini berisi 60 kategori sampah yang berbeda.

![YOLOv8](/assets/images/yolo.png)
![TACO](/assets/images/taco.png)

## model

Pada project ini penulis tidak melakukan training model YOLOv8, melainkan menggunakan model yang telah dilatih dari repositroi [berikut](https://github.com/jeremy-rico/litter-detection). Pada repository tersebut terdapat beberapa versi model YOLOv8 yang telah dilatih menggunakan dataset TACO yang dapat digunakan sebagai awalan untuk pembuktian konsep ini. Selain itu model pada project ini telah diubah menjadi format ONNX agar dapat digunakan pada bahasa pemrograman lain seperti Rust.

## pengujian

Untuk melakukan pengujian, penulis menggunakan [dataset TrashBox](https://github.com/nikhilvenkatkumsetty/TrashBox-testandvalid) yang terdiri dari tuju kelas sampah. Namun tidak semua kelas sampah dapat digunakan karena berbeda dengan kelas sampah yang terdapat pada dataset TACO. Untuk itu penulis hanya menggunakan kelas sampah yang terdapat pada dataset TACO dengan pemilihan manual. Tujuan dari penggunaan dataset yang berbeda untuk pengujian adalah untuk **Holdout Validation** atau pengujian model pada dataset yang berbeda dari dataset yang digunakan untuk training.

### cara pengujian

Untuk melakukan pengujian, penulis menggunakan script `evaluate_object.py` yang terdapat pada folder `examples`. Script ini akan memuat model `YOLOv8-m` pada folder `assets/model` dan melakukan prediksi pada gambar yang terdapat pada folder `assets/test`. Gambar yang terdapat pada folder `assets/test` akan diberi label berdasarkan nama folder tempat gambar tersebut berada. Hasil dari prediksi akan dibandingkan dengan label yang diberikan pada gambar tersebut.

#### prepare data

Sebelum digunakan sebagai input pada model, gambar terlebih dahullu harus diubah menjadi format yang dapat diterima oleh model. Perubahan yang dilakukan antara mengubah ukuran sesuai input model yaitu 640x640, mengubah warna gambar menjadi RGB, menjadikan intput sebagai satu batch, dan melakukan normalisasi pada gambar.

#### prediksi

Setelah data siap, model akan melakukan prediksi pada gambar yang telah disiapkan. Prediksi dilakukan dengan cara memasukkan gambar ke dalam model dan mendapatkan output dari model. Output dari model berupa bounding box yang menunjukkan posisi objek pada gambar, kelas objek, dan skor prediksi.

#### evaluasi

Setelah mendapatkan output dari model, output tersebut akan dibandingkan dengan label pada nama folder. Jika akurasi lebih dari 50% maka poin akurasi akan ditambahkan. Total akurasi akan dihitung dengan cara jumlah poin akurasi benar dibagi dengan jumlah data yang diuji.

## hasil

### 1. pengujian pada data kecil

#### data

```bash
➜  examples git:(example/python) ✗ tree ../assets/test
../assets/test
├── Other plastic bottle
│   └── test_image.jpg
└── Plastic bottle cap
    ├── 2024-07-03-165528.jpg
    ├── 2024-07-03-165749.jpg
    ├── 2024-07-03-165756.jpg
    ├── 2024-07-03-165803.jpg
    ├── 2024-07-03-165824.jpg
    └── 2024-07-03-165829.jpg

2 directories, 7 files
```

#### hasil

```bash
➜  examples git:(example/python) ✗ python3 evaluate_object.py
Processing Plastic bottle cap...
100%|███████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 6/6 [00:02<00:00,  2.15it/s]
Processing Other plastic bottle...
100%|███████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 1/1 [00:00<00:00,  3.86it/s]

Overall Accuracy: 71.43%

Class-wise Accuracy:
Plastic bottle cap: 83.33% (5/6)
Other plastic bottle: 0.00% (0/1)
```

### 2. pengujian pada data lebih banyak

#### data

![folder preview](/assets/images/val_folder_class.png)

#### hasil

Hasil saat dijalankan pada data yang lebih banyak. Hasil ini menunjukkan bahwa model yang digunakan memiliki akurasi yang rendah. Hal ini disebabkan oleh beberapa faktor, diantaranya adalah dataset yang digunakan tidak sesuai dengan dataset yang digunakan untuk training, model yang digunakan tidak sesuai dengan dataset yang digunakan, atau bahkan pengelompokan kelas data yang salah oleh penulis sendiri.

```bash
➜  examples git:(example/python) ✗ python3 evaluate_object.py
Processing Glass bottle...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 16/16 [00:04<00:00,  3.27it/s]
Processing Other plastic...
 70%|███████████████████████████████████████████████████████████████████████████████████████████████████████████▋                                             | 69/98 [00:12<00:04,  5.86it/s]libpng warning: iCCP: known incorrect sRGB profile
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 98/98 [00:17<00:00,  5.66it/s]
Processing Single-use carrier bag...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 47/47 [00:08<00:00,  5.55it/s]
Processing Plastic bottle cap...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 11/11 [00:02<00:00,  5.10it/s]
Processing Other plastic bottle...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 57/57 [00:11<00:00,  4.93it/s]
Processing Paper cup...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 80/80 [00:14<00:00,  5.67it/s]
Processing Cigarette...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 55/55 [00:09<00:00,  5.91it/s]
Processing Disposable food container...
100%|█████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████| 54/54 [00:09<00:00,  5.79it/s]

Overall Accuracy: 7.89%

Class-wise Accuracy:
Glass bottle: 0.00% (0/16)
Other plastic: 0.00% (0/98)
Single-use carrier bag: 6.38% (3/47)
Plastic bottle cap: 45.45% (5/11)
Other plastic bottle: 17.54% (10/57)
Paper cup: 7.50% (6/80)
Cigarette: 14.55% (8/55)
Disposable food container: 1.85% (1/54)
```
