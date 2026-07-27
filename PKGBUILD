pkgname=pae
pkgver=0.1.0
pkgrel=1
pkgdesc="Lightweight text editor built with GTK4 and Rust"
arch=('x86_64')
license=('GPL-3.0')
depends=('gtk4')

package() {

    install -Dm755 "$srcdir/../target/release/PAE" \
        "$pkgdir/usr/bin/PAE"

    install -Dm644 "$srcdir/../assets/applications/com.jayden.pae.desktop" \
        "$pkgdir/usr/share/applications/com.jayden.pae.desktop"

    install -Dm644 "$srcdir/../assets/icons/hicolor/512x512/apps/com.jayden.pae.png" \
        "$pkgdir/usr/share/icons/hicolor/512x512/apps/com.jayden.pae.png"

    install -Dm644 "$srcdir/../assets/metainfo/com.jayden.pae.metainfo.xml" \
        "$pkgdir/usr/share/metainfo/com.jayden.pae.metainfo.xml"
}