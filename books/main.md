# ゼロから作るMyOS
皆さんは、コンピュータの基本的な仕組みを理解していますか？
この本（本...？）では、ゼロから自分だけのオペレーティングシステム（OS）を作成する方法を学べます。

OSは、ハードウェアとソフトウェアの橋渡しをする重要な役割を果たしています。
OSを理解することによって、コンピュータの動作原理やプログラミングの基礎を深く学ぶことができますよ！

では、Let's make your own OS!

この本では、以下の内容をカバーします：
- OSの基本概念
- ブートローダーの作成
- カーネルでの「Hello, World!」の表示
- メモリ管理の基礎

## はじめに
この本を手に取ったあなたは、ある程度OSに興味があるのでしょう。
ですが、実際のところOSって何？という人も多いと思います。
なので、まずはOSの基本的な概念から説明します。

OSとは、コンピュータのハードウェア（CPUやメモリなど）とソフトウェア（Word, Chrome, カウンターストライクなどなど...）の間に位置するソフトウェアのことです。
OSは、ハードウェアのリソース（メモリなどの限られた資源）を管理し、アプリケーションがそれらのリソースを効率的に利用できるようにします。
例えば、OSはメモリをアプリに割り当てたり、ファイルシステム（これがないとtest.txtすら保存できません）の管理、マルチタスク（Youtube見たりしながら開発したり、ゲームしながらTwitter見たりするにはこれが必須）などを担当します。
まぁ、要は、OSはコンピュータの管制塔てきなものです。

...いや、OSについて理解はしたけどそもそもOSって開発できるんですか？と思うかもしれません。いいえ、全然可能です！

なぜなら、OSといってもここで作るのは非常にシンプルなものだからです。
実際のOSは非常に複雑で、多くの機能を持っていますが、ここでは基本的な部分に焦点を当てます。
この本を通じて、OSの基本的な仕組みを理解し、自分でOSを作成する楽しさを体験してください！

追記: この本は初心者向けに書かれていますが、ある程度のプログラミング経験（特にRustやアセンブリ言語）があると理解が深まります。

### 必要なツール
OS開発を始めるにあたって、いくつかのツールが必要です。
以下に、必要なツールとそのインストール方法を説明します。
> Windowsの場合、WSL（Windows Subsystem for Linux）を使用することをお勧めします。WSLを使用すると、Linux環境での開発が可能になります。詳細は[公式ドキュメント](https://docs.microsoft.com/ja-jp/windows/wsl/install)を参照してください。

1. **Rust**: Rustはモダンなシステムプログラミング言語で、OS開発に適しています。公式サイト（https://www.rust-lang.org/ja/tools/install）からインストールしてください。
2. **QEMU**: QEMUはオープンソースの仮想化ソフトウェア（PCの上にPCを構築することができます）で、OSのデバッグに使用します。公式サイト（https://www.qemu.org/download/）からインストールしてください。
3. お好きなテキストエディタ: Vim, Emacs, VScodeなど好きなエディタを使用してください。メモ帳でもgoodです :D

これらのツールをインストールしたら、次の章から実際にOS開発を始めましょう！

## 1. ブートローダーの作成
OSを起動するためには、まずブートローダーを作成する必要があります。
ブートローダーは、コンピュータの電源が入ったときに最初に実行されるプログラムで、OSのカーネルをメモリにロードし、実行します。
ですが、とりあえず「動く」というのを大事にするので、ここでは非常にシンプルなブートローダーを作成します。
本来ならば、ブートローダーは`/EFI/BOOT/BOOTX64.EFI`に配置する必要がある（UEFI仕様）ですが、ここではQEMUを使用するため、特別な配置は必要ありません。
めんどっちいことは抜きにして、さっそくコードを書いていきましょう！

まずはテンプレートをクローンしてきます。
```bash
git clone https://github.com/nekogakure/CreateMyOS.git myos
git fetch
git checkout init
cd myos
```

dependenciesとは、Rustで使用できるライブラリ（クレートと呼ばれます）を指定する場所です。ライブラリを追加することで、OS開発を効率的に、簡単に進めることができるようになります！

次に、ブートローダーを配置するためのディレクトリを`src/boot`として作成します。
そうしたら、`src/boot/loader.rs`というファイルを作成し、以下のコードを追加します。
```rust
#![no_std]
#![no_main]

use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi::helpers::init(&mut system_table) {
        return Status::UNSUPPORTED;
    }

    let _ = system_table.stdout().clear();
    let _ = system_table
        .stdout()
        .output_string(cstr16!("hello, world!\n"));

    loop {}
}
```

では、さっそく実行してみましょう！
```bash
cargo run
```

...あれれ？？

```bash
neko@nek0dev:~/Documents/myos$ cargo run
error: failed to parse manifest at `/home/neko/Documents/myos/Cargo.toml`

Caused by:
  no targets specified in the manifest
  either src/lib.rs, src/main.rs, a [lib] section, or [[bin]] section must be present
```

おっと、エラーが出てしまいましたね。
これは、Cargoがどのファイルを実行すれば良いのか分からないために発生しています。
そこで、`Cargo.toml`ファイルに以下の内容を追加します。

```toml
[[bin]]
name = "boot"
path = "src/boot/loader.rs"
```

見栄えのために、これは一番下に追加するのが良いでしょう。
これで、Cargoは`src/boot/loader.rs`ファイルを実行することができます。（`[[bin]]`セクションについてもっと知りたい場合は、[公式ドキュメント](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries)を参照してください。）

では、もう一度実行してみましょう！

```bash
cargo run
```

ありゃ、またエラーが出てしまいましたね。
```bash
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait

error: `#[panic_handler]` function required, but not found
```

このエラーは、Rustがメモリを管理するためのアロケータ（メモリ割り当てなどを行う仕組み）とパニックハンドラ（プログラムが予期しないエラーに遭遇したときに呼び出される関数）が見つからないために発生しています。
普段ならRustの標準ライブラリに含まれているのですが、OS開発では、標準ライブラリ（`std`）を使用できません。
そのため、独自のアロケータとパニックハンドラを実装する必要があります。
幸いなことに、`uefi`クレートにはこれらの実装が含まれているため、簡単に解決できます。
`src/boot/loader.rs`ファイルの先頭に以下の行を追加します。
```rust
use core::panic::PanicInfo;

// グローバルアロケータの設定
#[global_allocator]
static ALLOCATOR: uefi::alloc::Allocator = uefi::alloc::Allocator;

// パニックハンドラの実装
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```
これで、Rustは`uefi`クレートのアロケータと、独自実装したパニックハンドラを使用することができます。

では、もう一度実行してみましょう！（先ほどと同じコマンドです）
![hello](img/hello.png)

こんな感じで、QEMUのウィンドウが開き、「hello, world!」と表示されれば成功です！

お疲れさまでした！これで、最初の基礎的なOSの一部が完成しました。
次の章では、カーネルを作成して、「Hello, from MyOS!」と表示する方法を学びます。
ここまで理解できたあなたは素晴らしいと思います！ブートさせること自体が、一つ目の大きな山なので。