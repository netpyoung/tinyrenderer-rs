# 첫번째 시도

첫 레슨의 목표는 wire mesh를 렌더링하는 것입니다.
이를 위해서, 선을 그리는 법을 알아야합니다.
`Bresenham’s line algorithm` 를 읽고 넘어갈 수 도 있지만, 일단 코드를 짜보도록 합시다.
https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm

점(x0, y0)과 점(x1, y1)사이에 선을 그리는 가장 간단한 방법은 다음과 같습니다:


```rust
fn line(x0: i32, y0: i32, x1: i32, y1: i32, image : &mut Image , color : &Color) {
    // ref: https://www.rosettacode.org/wiki/Loops/For_with_a_specified_step#Rust

    let x0f = x0 as f32;
    let y0f = y0 as f32;
    let x1f = x1 as f32;
    let y1f = y1 as f32;

    let mut t : f32 = 0.0;
    while t < 1.0 {
        let x = (x0f * (1.0 - t) + x1f * t) as usize;
        let y = (y0f * (1.0 - t) + y1f * t) as usize;
        image.set_pixel(x, y, color);

        t += 0.01;
    }
}
```

    line(13, 20, 80, 40, &mut image, &color::RED);

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/01-bresenham/c3c2ea8819.png)

The snapshot of the code is available [here](https://github.com/ssloy/tinyrenderer/tree/d0703acf18c48f2f7d00e552697d4797e0669ade).

# 두번째 시도

효율성은 둘째치고 이전 코드의 문제점은, 0.01로 간격이 고정되어 있다는것 입니다.
0.1로 설정한다면, 다음과 같은 결과를 확인할 수 있을것 입니다.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/01-bresenham/62a16a5321.png)

저희는 쉽게 필요한 간격을 찾을 수 있습니다: 바로 그려져야할 픽셀의 수 입니다. 조금 부족한 면이 있지만, 다음과 같은 코드로 나타낼 수 있습니다.

```rust
fn line2(x0: i32, y0: i32, x1: i32, y1: i32, image : &mut Image , color : &Color) {
    let x0f = x0 as f32;
    let y0f = y0 as f32;
    let x1f = x1 as f32;
    let y1f = y1 as f32;

    let mut x : f32 = x0f;
    while x <= x1f {
        let t : f32 = (x - x0f) / (x1f - x0f);
        let y = y0f * (1.0 - t) + (y1f * t);
        image.set_pixel(x as usize, y as usize, color);

        x += 1.0;
    }
}
```

주의! 다음과 같이 integer를 그대로 연산하면 안됩니다 `(x - x0) / (x1 - x0)`.
이제, 다음 코드로 선을 그려보면:


```C++
line2(13, 20, 80, 40, &mut image, &color::WHITE);
line2(20, 13, 40, 80, &mut image, &color::RED);
line2(80, 40, 13, 20, &mut image, &color::RED);
```

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/01-bresenham/097a691f9e.png)


첫번째 선은 좋아보이지만, 두번째 선은 구멍이 나있고, 세번째는 보이지도 않습니다.
첫번째와 세번째를 보시면, 방향과 색깔만 달랐지 동일한 선을 그리려고 한다는 것을 확인할 수 있습니다.
흰선은 잘 그려졌습니다. 그리고 빨간선으로 덮으려고 했지만, 그렇지 못했습니다.
대칭성 테스트: 선을 그릴때에는 점의 순서에 영향을 받지 않는다: 선(a, b)은 선(b, a)와 같다.


# 세번째 시도

사라진 빨간선은 x0가 항상 x1보다 작도록 교환시켜주면 고칠 수 있습니다.


구멍이 난 선은, 사실 높이가 넓이보다 크다는 사실에 기인합니다.
제가 가르키는 학생들은 종종 다음과 같이 고치려고 했습니다:

```rust
if dx > dy {
  for (x)
} else {
  for (y)
}
```

이 자식. 안 되겠어. 빨리 어떻게든 하지 않으면..

```rust
fn line3(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    let mut steep = false;
    let mut x0f = x0 as f32;
    let mut y0f = y0 as f32;
    let mut x1f = x1 as f32;
    let mut y1f = y1 as f32;

    // if the line is steep, we transpose the image
    if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0f, &mut y0f);
        mem::swap(&mut x1f, &mut y1f);
        steep = true;
    }

    // make it left−to−right
    if (x0f > x1f) {
        mem::swap(&mut x0f, &mut x1f);
        mem::swap(&mut y0f, &mut y1f);
    }

    let mut x: f32 = x0f;
    while x <= x1f {
        let t: f32 = (x - x0f) / (x1f - x0f);
        let y = y0f * (1.0 - t) + (y1f * t);

        if (steep) {
            image.set_pixel(y as usize, x as usize, color);
        } else {
            image.set_pixel(x as usize, y as usize, color);
        }

        x += 1.0;
    }
}
```

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/01-bresenham/3e8e5c7d26.png)

# 시간: 네번째 시도

https://github.com/gperftools/gperftools

_**경고**: compiler’s optimizator (g++ -O3) is often better than you (and me) in creating a fast code. This section is here for historical/cultural reasons._

컴파일러의 최적화작업으로
(cargo build --release)
http://doc.crates.io/manifest.html#the-profile-sections
는 종종 저희보다 빠른 코드를 생성하기도 합니다.
그럼에도 불구하고, 이번 단락은 한번쯤 봐두면 좋습니다.


코드는 잘 동작합니다.
제가 원했던 정도의 복잡성을 지녔습니다.
나누기가 여러번 있어 비효율적이긴 하지만, 짧고 보기편합니다.

다만, 찜찜하게 asser랑 boundery체크가 없습니다.


이번시도에서는,
 In these articles I try not to overload this particular code, as it gets read a lot.
 At the same time, I systematically remind of the necessity to perform checks.


이전 코드는 잘 동작하지만, 최적화 여지가 남아있습니다.
최적화는 위험한 작업입니다.
그 코드가 해당 플랫폼에 잘 돌아가는지 확실히 해야합니다.
 We should be clear about the platform the code will run on.
그래픽카드나 CPU에 대한 코드최적화와는 완전히 다릅니다.

코드 최적화를 하기 앞서, 프로파일을 돌려야합니다.
여기서 어떤 작업이 자원을 가장 많이 잡아먹는지 생각해봅시다.


테스트를 위해 3개의 선을 1,000,000번 그려보았습니다.
제 CPU는 `Intel® Core(TM) i5-3450 CPU @ 3.10GHz`입니다.
각 픽셀 각각에 대해 Color객체의 복사가 이루어집니다.

 1000000 * 선 3개 * 픽셀(대략)50번 입니다.

호출이 많은것 같지 않습니까?
어디서부터 최적화를 해야할까요?
프로파일러가 그 답을 말해줍니다.


코드를 다음과 같이 컴파일하여, 프로파일을 하였습니다:

brew install gperftools

```
%   cumulative   self              self     total
 time   seconds   seconds    calls  ms/call  ms/call  name
 69.16      2.95     2.95  3000000     0.00     0.00  line(int, int, int, int, TGAImage&, TGAColor)
 19.46      3.78     0.83 204000000     0.00     0.00  TGAImage::set(int, int, TGAColor)
  8.91      4.16     0.38 207000000     0.00     0.00  TGAColor::TGAColor(TGAColor const&)
  1.64      4.23     0.07        2    35.04    35.04  TGAColor::TGAColor(unsigned char, unsigned char, unsigned char, unsigned char)
  0.94      4.27     0.04                             TGAImage::get(int, int)
```

색을 복사하는 것에는 10%정도 시간을 소비합니다. 하지만, line()을 호출하는데에는 70%나 소비합니다! 저기가 바로 최적화를 할 곳입니다.

# 네번째 시도 계속

동일한 분모로 나누셈을 한다는 것을 눈치채셨을 것입니다.
이를 loop밖으로 꺼내보도록 합시다.
error변수
We should note that each division has the same divisor. Let’s take it out of the loop. The error variable gives us the distance to the best straight line from our current (x, y) pixel.
error값이 하나의 픽셀보다 클때마다, y를 하나 증가(혹은 감소)시키고, error 또한 하나 감소시킵니다.
 Each time error is greater than one pixel, we increase (or decrease) y by one, and decrease the error by one as well.


The code is available [here](https://github.com/ssloy/tinyrenderer/tree/2086cc7c082f4aec536661d7b4ab8a469eb0ce06).

```C++
void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
    bool steep = false;
    if (std::abs(x0-x1)<std::abs(y0-y1)) {
        std::swap(x0, y0);
        std::swap(x1, y1);
        steep = true;
    }
    if (x0>x1) {
        std::swap(x0, x1);
        std::swap(y0, y1);
    }
    int dx = x1-x0;
    int dy = y1-y0;
    float derror = std::abs(dy/float(dx));
    float error = 0;
    int y = y0;
    for (int x=x0; x<=x1; x++) {
        if (steep) {
            image.set(y, x, color);
        } else {
            image.set(x, y, color);
        }
        error += derror;
        if (error>.5) {
            y += (y1>y0?1:-1);
            error -= 1.;
        }
    }
}
```


gprof 결과가 나와있습니다:

```
%   cumulative   self              self     total
 time   seconds   seconds    calls  ms/call  ms/call  name
 38.79      0.93     0.93  3000000     0.00     0.00  line(int, int, int, int, TGAImage&, TGAColor)
 37.54      1.83     0.90 204000000     0.00     0.00  TGAImage::set(int, int, TGAColor)
 19.60      2.30     0.47 204000000     0.00     0.00  TGAColor::TGAColor(int, int)
  2.09      2.35     0.05        2    25.03    25.03  TGAColor::TGAColor(unsigned char, unsigned char, unsigned char, unsigned char)
  1.25      2.38     0.03                             TGAImage::get(int, int)
```


# Timings: fifth and final attempt

floating point가 필요할까요?
loop에서 dx로 나누고, 0.5를 나누는 이유때문에요.
error변수를 다른걸로 바꿔서 floating point를 없앨 수 있습니다.
이를 error2로 하고, 이는 error * dx * 2라 합시다.

```C++
void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
    bool steep = false;
    if (std::abs(x0-x1)<std::abs(y0-y1)) {
        std::swap(x0, y0);
        std::swap(x1, y1);
        steep = true;
    }
    if (x0>x1) {
        std::swap(x0, x1);
        std::swap(y0, y1);
    }
    int dx = x1-x0;
    int dy = y1-y0;
    int derror2 = std::abs(dy)*2;
    int error2 = 0;
    int y = y0;
    for (int x=x0; x<=x1; x++) {
        if (steep) {
            image.set(y, x, color);
        } else {
            image.set(x, y, color);
        }
        error2 += derror2;
        if (error2 > dx) {
            y += (y1>y0?1:-1);
            error2 -= dx*2;
        }
    }
}
```

```
%   cumulative   self              self     total
 time   seconds   seconds    calls  ms/call  ms/call  name
 42.77      0.91     0.91 204000000     0.00     0.00  TGAImage::set(int, int, TGAColor)
 30.08      1.55     0.64  3000000     0.00     0.00  line(int, int, int, int, TGAImage&, TGAColor)
 21.62      2.01     0.46 204000000     0.00     0.00  TGAColor::TGAColor(int, int)
  1.88      2.05     0.04        2    20.02    20.02  TGAColor::TGAColor(unsigned char, unsigned char, unsigned char, unsigned char)
```

색을 레퍼런스로 넘김으로써 불필요한 복사가 일어나는 것을 없에도록 합시다.
Now, it’s enough to remove unnecessary copies during the function call by passing the color by reference (or just enable the compilation flag -O3), and it’s done.

 Not a single multiplication or division in code. The execution time has decreased from 2.95 to 0.64 seconds.

# 와이어프레임 렌더링

와이어 렌더더를 만들어 봅시다.
So now we are ready to create a wire render. You can find the snapshot of the [code and the test model here](https://github.com/ssloy/tinyrenderer/tree/f6fecb7ad493264ecd15e230411bfb1cca539a12).

다음과 같은 모델 파일을 이용했습니다.
I used the [wavefront obj](http://en.wikipedia.org/wiki/Wavefront_.obj_file) format of the file to store model.
렌더러에 필요한 것은 다음과 같이 선분 배열로 구성된 파일을 읽어들이는 것입니다:


```
v 0.608654 -0.568839 -0.416318
```

x, y, z좌표가 될 것이고, 정점과 면
are x,y,z coordinates, one vertex per file line
and faces
```
f 1193/1240/1193 1180/1227/1180 1179/1226/1179
```

We are interested in the first number after each space. It is the number of the vertex in the array that we have read before. Thus, this line says that 1193, 1180 and 1179 vertices form a triangle.
The model.cpp file contains a simple parser. Write the following loop to our main.cpp and voilà, our wire renderer is ready.

```rust
for face in model.faces.iter() {
    for j in 0..3 {
        let v0 = model.vert(face[j]);
        let v1 = model.vert(face[(j+1) % 3]);
        let x0 = ((v0.x + 1.0) * width as f32 / 2.0) as i32;
        let y0 = ((v0.y + 1.0) * height as f32  / 2.0) as i32;
        let x1 = ((v1.x + 1.0) * width as f32  / 2.0) as i32;
        let y1 = ((v1.y + 1.0) * height as f32  / 2.0) as i32;
        line3(x0, y0, x1, y1, &mut image, &color::WHITE);
    }
}
```
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/01-bresenham/5da6818190.png)

다음으로는 2D 삼각형을 그려보도록 하겠습니다.
