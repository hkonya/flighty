# Genel
app-name = Flighty
app-description = Flutter uygulama dağıtım ve yönetim CLI aracı

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