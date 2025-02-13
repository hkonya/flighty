# Genel
app-name = Flighty
app-description = Flutter Dağıtım Aracı
app-version = v1.0.0

# Init komutu
init-prompt = Bu projeyi nasıl adlandıralım?
init-default-name = my_flutter_app

# Proje adı hataları
project-name-length-error = Proje adı 3-30 karakter arasında olmalıdır
project-name-start-error = Proje adı sayı ile başlayamaz
project-name-chars-error = Sadece küçük harf, sayı ve alt çizgi kullanabilirsiniz

# İşlem adımları
step-check-flutter = Flutter projesi kontrol ediliyor...
step-create-structure = Dizin yapısı hazırlanıyor...
step-update-git = Git yapılandırması güncelleniyor...
step-create-config = Yapılandırma dosyaları oluşturuluyor...

# Başarı mesajları
success-init = Flighty başarıyla kuruldu!
success-app-created = Flighty uygulaması oluşturuldu.
success-config-created = 'flighty.yaml' dosyası oluşturuldu.
success-pubspec-updated = 'pubspec.yaml' dosyası güncellendi.

# Komut açıklamaları
commands-intro = Başlamak için aşağıdaki komutları kullanabilirsiniz:
command-build = Yeni bir sürüm oluşturmak için
command-deploy = Güncelleme göndermek için
command-preview = Önizleme yapmak için

# Hata mesajları
error-title = Hata Detayı:
error-solution = Çözüm:
error-flutter-not-found = Flutter projesi bulunamadı
error-flutter-sdk-not-found = Flutter SDK bulunamadı
error-structure-failed = Dizin yapısı oluşturulamadı
error-git-failed = Git yapılandırması güncellenemedi
error-config-failed = Yapılandırma dosyaları oluşturulamadı

# Çözüm adımları
solution-create-project = Önce bir Flutter projesi oluşturun:
solution-install-sdk = Flutter SDK kurulumunu yapın:
solution-check-flutter = Flutter kurulumunuzu kontrol edin:

# Dokümantasyon
docs-more-info = Daha fazla bilgi için:

# Komutlar
cmd-init = Yeni bir proje başlat
cmd-init-path = Proje dizini

cmd-config = Yapılandırmayı görüntüle veya düzenle
cmd-config-key = Yapılandırma anahtarı
cmd-config-value = Yeni değer

cmd-build = Uygulamayı derle
cmd-build-platform = Derleme platformu (android/ios)
cmd-build-version = Sürüm numarası
cmd-build-number = Build numarası
cmd-build-channel = Release kanalı (alpha/beta/production)

cmd-deploy = Uygulamayı dağıt
cmd-deploy-platform = Dağıtım platformu (android/ios)
cmd-deploy-target = Dağıtım hedefi (store/firebase)

cmd-flutter = Flutter SDK'yı yönet
cmd-flutter-command = Flutter komutu

cmd-language = Dili değiştir
cmd-language-code = Dil kodu (en/tr)

# Mesajlar
msg-project-init = Proje başlatılıyor: {$path}
msg-config-exists = Yapılandırma dosyası zaten mevcut
msg-config-created = Varsayılan yapılandırma oluşturuldu
msg-flutter-missing = Flutter projesi bulunamadı. Lütfen önce 'flutter create' komutunu çalıştırın
msg-build-start = Build başlatılıyor: {$platform}
msg-build-complete = Build tamamlandı: {$platform}
msg-deploy-start = Dağıtım başlatılıyor: {$platform} -> {$target}
msg-deploy-complete = Dağıtım tamamlandı: {$platform}
msg-invalid-platform = Geçersiz platform
msg-invalid-target = Geçersiz hedef
msg-invalid-channel = Geçersiz kanal
msg-flutter-not-found = Flutter SDK bulunamadı
msg-language-changed = Dil değiştirildi: {$lang}

# Hatalar
error-config = Yapılandırma hatası: {$message}
error-flutter = Flutter CLI hatası: {$message}
error-io = IO hatası: {$message}
error-invalid-arg = Geçersiz argüman: {$message}
error-unknown = Bilinmeyen hata: {$message}

# Debug ve uyarı mesajları
debug-flutter-doctor = Flutter doctor çalıştırılıyor...
warn-android-missing = Android dizini bulunamadı
warn-ios-missing = iOS dizini bulunamadı

# Hata mesajları
error-message = Hata: {$message}
error-unknown-occurred = Beklenmeyen bir hata oluştu
error-flutter-doctor-failed = Flutter doctor başarısız oldu. Lütfen Flutter kurulumunuzu kontrol edin.

# Yapılandırma
config-current = Mevcut yapılandırma:
config-language = Dil: {$lang}
config-show-not-implemented = Yapılandırma değeri gösterme henüz uygulanmadı
config-set-not-implemented = Yapılandırma değeri ayarlama henüz uygulanmadı 