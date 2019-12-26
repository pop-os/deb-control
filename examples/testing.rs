//! Parser testing

#[macro_use]
extern crate fomat_macros;

use deb_control::prelude::*;

use futures::{executor, io::AllowStdIo, prelude::*};
use futures_codec::FramedRead;
use std::{io::Cursor, str};

const RELEASE: &str = r#"Architectures: amd64 i386
Codename: eoan
Components: main
Date: Tue, 24 Dec 2019 20:33:57 +0000
Description: Pop!_OS Staging eoan 19.10 master
Label: Pop!_OS Staging master
Origin: pop-os-staging-master
Suite: eoan
Version: 19.10
Folded: one two three
 four five
 six seven
MD5Sum:
 c4db6e9cc586d1f751f827600275be44           267707 main/binary-amd64/Packages
 046ff8b2085d56c2df942c94a149221e            69133 main/binary-amd64/Packages.gz
 ad26a9bd1692c6279cbc9f2534fc0c29              125 main/binary-amd64/Release
 41f704ca5292ab484294e3cf273d3094           127660 main/binary-i386/Packages
 ae8d1ef853f8f9a530287f6994a9207d            37775 main/binary-i386/Packages.gz
 09000b982cb1f5e1bc9a5d91eea8656c              124 main/binary-i386/Release
 2503116186791836ca99aae3b27e11e0              126 main/source/Release
 5d8f656b31b5de498641033d4ba33c09           122953 main/source/Sources
 f830ef2921adecdbffe0ce1389e4de56            33794 main/source/Sources.gz
SHA1:
 0e3ec3a8f4feb65139c38f0317417455a62fe331           267707 main/binary-amd64/Packages
 0510c90019b2a5cc7852ae9f040795fcc34947b1            69133 main/binary-amd64/Packages.gz
 fd6461ac74511639a82d56936b7baa7b9a48f55f              125 main/binary-amd64/Release
 28fb8e56629a167cf2f43691312de3dec5c00c57           127660 main/binary-i386/Packages
 3e466d26ae8dc74072d721159fb0a8360a5a98c3            37775 main/binary-i386/Packages.gz
 ea8aed3332eb4b0decd4df7804da85adc6a83b30              124 main/binary-i386/Release
 37305a59b7c11ea01d362dc0d6ef675a5c4a6773              126 main/source/Release
 c66e93fcb3d50a75b0bdc3207edd123d48a970cc           122953 main/source/Sources
 63ddd0eaa340b8c51d325697ad425a3ae6d306b4            33794 main/source/Sources.gz
SHA256:
 b0234de1fab04c2ee5b85d2de9f2c18e35169d415439f2fa0241e3c78a8ede54           267707 main/binary-amd64/Packages
 43f7900a7da9ba1809b6b2851bee1b7485b83f4547141de3a5d48a21bcb8cb7f            69133 main/binary-amd64/Packages.gz
 f5a8bae7378753d5d15c9f419a8927cd56752538b35ce914cf5216beb89a3547              125 main/binary-amd64/Release
 69a4f37bb941201504c0f01463863542a3f230710d93ce18903e85a499c30714           127660 main/binary-i386/Packages
 8a63e2c660646a66583f68914e5aaead76593c1fe6801f01b03d6b597b62498c            37775 main/binary-i386/Packages.gz
 d05f6371ebfff5e7ed68899fee457f406945d4e24ce5533fce232b6dc628224c              124 main/binary-i386/Release
 69e688eb11595c899a0d0858367df055ad1a694b230d7ac84a5bc53a478444b4              126 main/source/Release
 3b83d38cbf7a913603c467b77714071ee3dcc8379b86ed2b3ea77af45134e299           122953 main/source/Sources
 f35931cc06d950d3b03f5fbbd543d144531a50c4378b253ff825d90ecce3327d            33794 main/source/Sources.gz
SHA512:
 b18d7363c7ef63c488f614cf2177c92861dd3ad24e9e6d8eaab659b9e1cc64e848ec8e222cebe44694b23c2512ccdc78e5cd99524f360079afa77671b4f6bf1d           267707 main/binary-amd64/Packages
 f0569e194d0cbf36fe6ed989f5476399362105a8d4b094be41e923be80cc1048f88cf73eee6c9195f14e300365acb9e7442c0b9a35ca304b9f60363a200a6dcf            69133 main/binary-amd64/Packages.gz
 62dd456bb931837291fd62fc5cad7b71ae5d47f5d3dfb7ee02e8b1a0a139e753cf9857e0918275807bc49f62d501b93cafe969ebd472440e2bc363063ce8d5f8              125 main/binary-amd64/Release
 3ab369bdae533a1568737f46e11f9b075a9e69aa98244b6f39c98751338912bcf685d142d2761637c19f326385916d1c594e0feb40b8619b0e349b5c9a7fff9e           127660 main/binary-i386/Packages
 e258ece1c151d2163e7d9eb7020647a776651e8aa9f26a2bcf0cd113c1461b4942704d238e9cba0488907528c3df4fa91c2e025e5a876ea3a36c73c4305543d2            37775 main/binary-i386/Packages.gz
 39fddb027db82beec4b34fa8e6a7e6370dac8a42647e322f2e7656f3c2627bf9461ef533903ec71f756ee623ddd9dd384ce274dd6d1b00dc560f58c6cd3712cd              124 main/binary-i386/Release
 643f8e45ca8017cf25fc2ca312d4f900fe2a67284ba7c5a66844c6d70a1a63df0bea5bb2a7a9820a04eae1a616c120e84a5981958ee3e6f350d4d05ab7e52dbd              126 main/source/Release
 11c45b789c996f8f2f95ff3bd4f5a570782d8a648e2cb523e1675ae03b63ac48e5f26f210fd0e8d41ddc91d9457d0f9e0781451f9b83db39f4a56b374a95516e           122953 main/source/Sources
 c2766462f63f16d6935dd48440b4f5d72cbd469abe4d8e8196ae2b45c8652037889613d9e6e4c6c46dfb65680092893f6b3cdcae97f393fe365dc03c853766ab            33794 main/source/Sources.gz"#;

const STATUS: &str = r#"Package: accountsservice
Status: install ok installed
Priority: optional
Section: admin
Installed-Size: 586
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Architecture: amd64
Version: 0.6.55-0ubuntu10pop0
Depends: dbus, libaccountsservice0 (= 0.6.55-0ubuntu10pop0), libc6 (>= 2.4), libglib2.0-0 (>= 2.44), libpolkit-gobject-1-0 (>= 0.99)
Suggests: gnome-control-center
Conffiles:
 /etc/dbus-1/system.d/org.freedesktop.Accounts.conf 06247d62052029ead7d9ec1ef9457f42
Description: query and manipulate user account information
 The AccountService project provides a set of D-Bus
 interfaces for querying and manipulating user account
 information and an implementation of these interfaces,
 based on the useradd, usermod and userdel commands.
Homepage: https://www.freedesktop.org/wiki/Software/AccountsService/
Original-Maintainer: Debian freedesktop.org maintainers <pkg-freedesktop-maintainers@lists.alioth.debian.org>

Package: acl
Status: install ok installed
Priority: optional
Section: utils
Installed-Size: 188
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Architecture: amd64
Multi-Arch: foreign
Version: 2.2.53-4
Depends: libacl1 (= 2.2.53-4), libc6 (>= 2.14)
Description: access control list - utilities
 This package contains the getfacl and setfacl utilities needed for
 manipulating access control lists. It also contains the chacl IRIX
 compatible utility.
Original-Maintainer: Guillem Jover <guillem@debian.org>
Homepage: https://savannah.nongnu.org/projects/acl/

Package: acpi
Status: install ok installed
Priority: optional
Section: utils
Installed-Size: 43
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Architecture: amd64
Version: 1.7-1.1
Depends: libc6 (>= 2.7)
Description: displays information on ACPI devices
 Attempts to replicate the functionality of the 'old' apm command on
 ACPI systems, including battery and thermal information. Does not support
 ACPI suspending, only displays information about ACPI devices.
Original-Maintainer: Debian Acpi Team <pkg-acpi-devel@lists.alioth.debian.org>
Homepage: http://sourceforge.net/projects/acpiclient"#;

fn main() {
    better_panic::install();

    pintln!(
        for entry in Control::new(RELEASE) {
            (entry.key) ": " (entry.value) "\n"
        }
    );

    pintln!();

    let input = AllowStdIo::new(Cursor::new(STATUS));
    let mut stream = FramedRead::new(input, ControlDecoder::default());

    executor::block_on(async move {
        while let Some(event) = stream.next().await {
            let event = event.unwrap();
            let event = str::from_utf8(&event).expect("not UTF8");

            pintln!(
                "Package {\n"
                for entry in Control::new(&event) {
                    "\t" (entry.key) ": " (entry.value) "\n"
                }
                "}"
            )
        }
    })
}
