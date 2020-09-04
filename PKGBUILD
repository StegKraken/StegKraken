# Maintainer: bee

pkgname=stegkraken
_pkgname=StegKraken
pkgver=1.0.0
pkgrel=1
pkgdesc="The World's Fastest Stegcracker"
arch=("x86_64" "i686")
url="https://github.com/stegkraken/stegkraken"
license=("MIT")
provides=('stegkraken')
conflicts=('stegkraken')
depends=("steghide")
makedepends=("gcc")
source=("${pkgname}-${pkgver}.tar.gz::${url}/archive/${pkgver}.tar.gz")
sha256sums=('050fb291563bcd4e3578e9d26459b044c152bde6622abf4294b55f9341b43342')

build() {
  cd ${_pkgname}-${pkgver}
  g++ src/main.cpp -std=c++20
}

package() {
  cd ${_pkgname}-${pkgver}
  install -Dm755 target/release/${pkgname} ${pkgdir}/usr/bin/${pkgname}
}