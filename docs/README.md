TODO: https://toolchain.gitbook.com/languages.html

origin: https://github.com/ssloy/tinyrenderer/wiki
ref: https://github.com/xymostech/rust-sdr


2017.03.15

번역은 초월번역도 상관없게 너무 형식에 얽매이지 않도록. 훗날 원본이 갱신되더라도, 굳이 원본에 딱 맞지 않더라도, 재량것 개선(의역)해 나가면 좋을것같다.

원래 코드는 C++ 기반으로 되어있다. rust공부겸 렌더링 공부겸 겸사겸사 진행한거라 rust로 작성한 코드가 별로 일 수 도 있다. 피드백은 언제나 환영한다.

하루에 반장씩 진행한것같다. rust도 배우면서 코드짜랴, 번역하랴, 내용 이해하랴. 처음 hook이 중요. '할 수 있다'라고 되내기며 하자.


How OpenGL works: software rendering in 500 lines of code

제 소스 코드는 상관없습니다. 위키를 읽고, 여러분만의 렌더러를 구현하세요. 여러분이 구석구석 탐구할때, 비로써 깨닫게 될것입니다.

저는 피드백을 환영합니다(dmitry.sokolov@univ-lorraine.fr); 질문이 있으시면 주저하시지 말고 연락주시기 바랍니다.

선생님께서 이 자료를 수업자료로 활용하길 원하신다면, 얼마든지 환영합니다. 얼마든지 활용해주세요. 제게 메일을 주셔서 알려주시면, 이 과정을 개선하는데 도움이 될것입니다.


이 시리즈에서, 저는 OpenGL이 동작하는 방식을 보여주고자 합니다.
놀랍게도 저는, OpenGL / DirectX를 공부함에 있어, 처음 난관을 극복하지 못한 사람들을 많이 만났습니다.
그러므로, 저는 짧은 교육자료들을 준비하였으며, 곧 제 제자들은 좋은 렌더더를 만들어 보여주었습니다.

그럼, 저희가 할 일은 다음과 같습니다 : 서드파티 라이브러리를 사용하지 않고(특히 그래픽 관련된 것들), 다음과 같은 그림을 얻어낼 것입니다.



주의:
이 교육자료를 활용하여 OpenGL 라이브러리의 구조를 대략적으로 살펴볼 것입니다.
소프트웨어 렌더러를 만들것입니다.
OpenGL 어플리케이션을 작성하는 것이 아니라, OpenGL이 어떻게 동작하는지 보여드리고 싶습니다.
저는 이것을 이해하지 않고는, 3D라이브러리를 이용하여 효율적인 어플리케이션을 작성하는것은 불가능에 가깝다고 확신합니다.

500줄 남짓한 코드를 작성할것입니다. 다음과 같은 렌더러를 만들기 위해, 제 제자들은 10 ~ 20시간 가까이 걸렸습니다.

폴리곤 라인과 텍스쳐파일을 입력으로, 렌더된 모델을 결과로 얻을것입니다.
그래픽 인터페이스는 없고, 단순히 이미지를 생성해 낼 것입니다.


외부 의존성을 최소화하기 위해, 저는 학생들에게 TGA파일을 다룰 수 있는 클래스만 제공해 주었습니다.
TGA는 RGB/RGBA/black/white를 지닌 이미지를 지원하는 매우 간단한 포맷중 하나입니다.

그러므로, 처음에는 이미지를 먼저 간단히 다뤄보도록 하겠습니다.
처음에는 하나의 픽셀에 색을 변경하는 것만 가능합니다.

선을 그리거나, 삼각형을 그리는 함수는 없습니다.
전부 처음부터 작성할 것입니다.

저는 제가 작성한 코드들을 학생들 각각에게 나누어 주었지만, 이것을 활용하는것은 권하지 않았습니다.
전체 코드는 github에서 얻으실 수 있습니다. 다음은 제가 학생들에게 건내준 코드입니다.


```
imagefmt라이브러리를 사용하고 base로 래핑하였습니다.

rustup
$ curl https://sh.rustup.rs -sSf | sh
$ rustup update
$ rupstup component add rust-src

$ rustup --version
rustup 0.5.0 (4be1012 2016-07-30)
$ rustc --version
rustc 1.15.1 (021bd294c 2017-02-08)
$ cargo --version
cargo 0.16.0-nightly (6e0c18c 2017-01-27)
$ cargo new prj --bin
 tree
.
└── prj
    ├── Cargo.toml
    └── src
        └── main.rs

intellj - intellij-rust를 활용하면 좋습니다.

clippy - lint
cargo install rustfmt # formmatter


# 잠깐만
std::process::exit(code: i32) is the way to exit with a code.
You can set the return value with std::os::set_exit_status.

http://stackoverflow.com/questions/24245276/why-does-rust-not-have-a-return-value-in-the-main-function-and-how-to-return-a


```rust
extern crate imagefmt;
mod base;
use base::image::{Image, IImage};
use base::color;

fn main() {
    let (width, height) = (100, 100);
    
    let mut image = Image::new(width, height);
    image.set_pixel(52, 41, &color::RED));
    image.write("out.tga").unwrap();
}
```

`output.tga`는 다음과 같이 보여질 것입니다:



Teaser: few examples made with the renderer
