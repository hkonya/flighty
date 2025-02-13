# Flighty

Flutter uygulama dağıtım ve yönetim CLI aracı.

## Özellikler

- 🚀 Hızlı ve kolay Flutter uygulama derleme
- 📱 Android ve iOS platformları için destek
- 🔄 Otomatik versiyon yönetimi
- 🎯 Firebase App Distribution entegrasyonu
- 🏪 App Store ve Google Play dağıtım desteği
- ⚙️ Yapılandırılabilir derleme ve dağıtım ayarları

## Kurulum

```bash
# Cargo ile kurulum
cargo install --path .

# veya doğrudan binary olarak
cargo build --release
```

## Kullanım

```bash
# Yeni bir proje başlat
flighty init

# Yapılandırmayı görüntüle
flighty config

# Yapılandırmayı güncelle
flighty config key value

# Uygulamayı derle
flighty build --platform android --version 1.0.0 --build-number 1 --channel beta

# Uygulamayı dağıt
flighty deploy --platform android --target firebase

# Flutter komutlarını çalıştır
flighty flutter doctor
```

## Yapılandırma

`.flighty.yaml` dosyası ile projenizi yapılandırabilirsiniz:

```yaml
flutter:
  sdk_path: /path/to/flutter
  version: 3.19.0

android:
  keystore_path: /path/to/keystore.jks
  keystore_password: password
  key_alias: alias
  key_password: password

ios:
  bundle_id: com.example.app
  team_id: TEAM_ID
  provisioning_profile: /path/to/profile
  certificate: /path/to/certificate

build:
  release_channel: beta
  version_strategy: semver
  build_number: 1
```

## Geliştirme

```bash
# Bağımlılıkları yükle
cargo build

# Testleri çalıştır
cargo test

# Formatla
cargo fmt

# Lint kontrolü
cargo clippy
```

## Lisans

MIT License - Detaylar için [LICENSE](LICENSE) dosyasına bakın. 