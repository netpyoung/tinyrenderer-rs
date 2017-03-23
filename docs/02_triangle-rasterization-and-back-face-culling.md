# 삼각형 채우기

안녕하세요, 접니다.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/cfa0f3a9d9.png)

정확히는, 저희가 앞으로 한 두시간에 걸쳐 만든 것으로 렌더링하게 될 제 얼굴 모형입니다.
저번 시간에는 3차원 모델의 메쉬와이어를 그렸습니다.
이번 시간에는 폴리곤을 채운다라기 보다는, 삼각형을 채울 것입니다.
실제로도, OpenGL은 폴리곤을 삼각형으로 구성하기에, 복잡하게 생각할 필요가 없습니다.

_다시 말씀드리지만, 여기 연재하는 목적은 여러분만의 프로그램을 만들 수 있도록 하는 것입니다.
제가 두시간에 걸쳐 여러분이 위와 같은 사진을 그릴 수 있다고 말한것은, 제 코드를 읽으라는 것이 아닙니다.
밑바닦부터 여러분의 코드를 작성할 때라는 것입니다.
여기 작성한 제 코드는 여러분이 작성할 코드와 비교할 목적으로 남겨두었습니다.
저는 그리 훌륭한 프로그래머는 아니므로, 여러분이 저보다 보다 더 나은 코드를 작성할 수 있습니다.
제 코드를 단순히 복사-붙여넣기 하지 마세요. 어떠한 질문이나 의견이라도 환영합니다_



## 옛날 방식: Line sweeping
https://en.wikipedia.org/wiki/Sweep_line_algorithm
사실, 이 작업은 2차원 삼각형을 그리기 위한 것입니다.
 좀 의욕적인 학생이 이걸 만든다면, 프로그램을 잘 못 짜더라도, 얼핏잡아 한 두시간이면 짭니다.
지난번에는 Bresenham의 line drawing algorithm을 봤습니다.
오늘의 목표는 삼각형을 색칠하는 것입니다.
간단하지만, 간단하지 않습니다.
사실이 그렇습니다.
이 간단한 작업에 제 많은 학생들이 고생하였습니다.
처음 얼개는 다음과 같습니다:


```rust
fn line4(p0: &Vec2<i32>, p1: &Vec2<i32>, image: &mut Image, color: &Color) {
    line3(p0.x, p0.y, p1.x, p1.y, image, color);
}

fn triangle(p0: &Vec2<i32>, p1: &Vec2<i32>, p2: &Vec2<i32>, image: &mut Image, color: &Color) {
    line4(p0, p1, image, color);
    line4(p1, p2, image, color);
    line4(p2, p0, image, color);
}

// ...
let t0 = [Vec2::new(10, 70), Vec2::new(50, 160), Vec2::new(70, 80)];
let t1 = [Vec2::new(180, 50), Vec2::new(150, 1), Vec2::new(70, 180)];
let t2 = [Vec2::new(180, 150), Vec2::new(120, 160), Vec2::new(130, 180)];
triangle(&t0[0], &t0[1], &t0[2], &mut image, &color::RED);
triangle(&t1[0], &t1[1], &t1[2], &mut image, &color::WHITE);
triangle(&t2[0], &t2[1], &t2[2], &mut image, &color::GREEN);
```

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/41060d3251.png)

간단한 코드입니다: 디버깅 목적으로 일단 3개의 삼각형을 만들었습니다.
triangle함수안에서 `line()`을 호출하면, 삼각형의 윤곽을 확인할 수 있을 것입니다.
하지만 어떻게 삼각형 안을 채워놓을까요?


삼각형을 그리기 위해서는 다음과 같은 원칙을 반드시 따라야 합니다:
A good method of drawing a triangle must have the following features:

* (엄청) 단순하면서 빨라야합니다.
* 버텍스의 순서에 영향을 받지 않아야 합니다.
* 두 삼각형이 두 개의 공통된 버텍스를 가질때, rasterization rounding으로 이 둘 사이에 구멍이 생기지 않아야 합니다.

* 요구사항을 더 추가할 수 도 있지만, 일단 이것까지만 합시다. 전통적으로, line sweeping방식은 다음과 같습니다:


1. y좌표로 버텍스를 정렬한다;
2. 삼각형의 좌우측을 동시에 레스터화한다;
3. 좌우 경계사이에 가로 선을 그린다.


여기서 제가 가르치는 학생들은 방황하기 시작합니다:
어떤 선분이 왼쪽이고 오른쪽일까요?
삼각형에는 3개의 선분이 있습니다... 이렇게 설명한 후 저는 학생들을 1시간 동안 지켜봅니다: 다시한번 말씀드리지만, 제 코드를 읽는것보다 직접 짜시는게 보다 가치있을것입니다.


[한시간이 흐른 뒤]

저는 어떻게 삼각형을 그렸을까요? 다시한번 말씀드리지만, 보다 더 나은 방법이 있다면, 얼마든지 환영합니다.
삼각형을 구성하는 3개의 점이 있다고 합시다:
t0, t1, t2는 y좌표로 정렬합니다.
바운더리 A는 t0과 t2, 바운더리 B는 t0과 t1, 그 다음은 t1과 t2입니다.

```rust
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Ord> Ord for Vec2<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ycmp = self.y.cmp(&other.y);
        match  ycmp {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => ycmp,
        }
    }
}

fn triangle(p0: &Vec2<i32>, p1: &Vec2<i32>, p2: &Vec2<i32>, image: &mut Image, color: &Color) {
    let mut v = vec![p0, p1, p2];
    v.sort();
    {
        let (p0, p1, p2) = (v[0], v[1], v[2]);
        line4(p0, p1, image, &color::GREEN);
        line4(p1, p2, image, &color::GREEN);
        line4(p2, p0, image, &color::RED);
    }
}
```

바운더리 A는 빨강으로, 바운더리 B은 초록색으로 하였습니다.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/3a5643f513.png)

유감스럽게도, 바운더리 B는 두 부분으로 이루어졌습니다.
가로로 반으로 잘라보도록 하겠습니다:

```rust
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;
    for y in v0.y as i32..(v1.y as i32 + 1) {
        let segment_height = v1.y - v0.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v0.y) / segment_height;
        let A = v0 + ((v2 - v0) * alpha);
        let B = v0 + ((v1 - v0) * beta);

        image.set_pixel(A.x as usize, y as usize, &color::RED);
        image.set_pixel(B.x as usize, y as usize, &color::GREEN);
    }
}
```

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/d8e0575a00.png)

선분이 이어지지 않았다는 점을 주목하세요.
지난번 직선을 그릴때에는, 연속된 선을 얻으려고 고생했지만, 여기서는 그러지 않았습니다. 나중에 삼각형을 채워넣을 것이기 때문입니다. 점들을 가로선으로 이어주기만 하면 틈이 사라질 것 입니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/c1f95127ad.png)

자, 이제 나머지 반쪽 삼각형을 그려보도록 합시다. 루프를 하나 더 추가하면 될 것같습니다:

```rust
fn triangle(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;
    for y in v0.y as i32..(v1.y as i32 + 1) {
        let segment_height = v1.y - v0.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v0.y) / segment_height;
        let mut A = v0 + ((v2 - v0) * alpha);
        let mut B = v0 + ((v1 - v0) * beta);
        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..B.x as i32 + 1 {
            image.set_pixel(j as usize, y as usize, color);
        }
    }

    for y in v1.y as i32..(v2.y as i32 + 1) {
        let segment_height = v2.y - v1.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v1.y) / segment_height;
        let mut A = v0 + ((v2 - v0) * alpha);
        let mut B = v1 + ((v2 - v1) * beta);
        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..(B.x as i32 + 1) {
            image.set_pixel(j as usize, y as usize, color);
        }
    }
}
```

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/b1a0fce5f1.png)

이걸로도 충분하지만, 저는 동일한 코드가 반복되는것을 싫어합니다.
가독성이 쪼금 떨어질지는 모르지만, 수정/관리가 더 쉽게 만들것입니다.


```rust
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;

    for i in 0..total_height as i32 {
        let i: f32 = i as f32;

        let alpha = i / total_height;
        let mut A = v0 + (v2 - v0) * alpha;
        let mut B: Vec2f;

        if (i > v1.y - v0.y) || (v1.y == v0.y) {
            let segment_height = v2.y - v1.y;
            let beta = (i - (v1.y - v0.y)) / segment_height;
            B = v1 + (v2 - v1) * beta;
        } else {
            let segment_height = v1.y - v0.y;
            let beta = i / segment_height;
            B = v0 + (v1 - v0) * beta;
        }

        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..B.x as i32 + 1 {
            image.set_pixel(j as usize, (v0.y + i) as usize, color);
        }
    }
}
```

[Here’s the commit](https://github.com/ssloy/tinyrenderer/tree/024ad4619b824f9179c86dc144145e2b8b155f52) for drawing 2D triangles.



## The method I adopt for my code

막 복잡하지는 않지만, line sweeping 코드는 조금 지저분합니다.
더욱이, 이는 mono-thread CPU 프로그래밍에 맞게 고안된 옛날 방식입니다.
다음 의사코드를 보도록 합시다:

```rust
fn triangle(points: &[i32]) {
    assert_eq!(points.len(), 3);

    let bounding_boxs = find_bounding_box(points);
    for bounding_box in bounding_boxs {
        for pixel in bounding_box {
            if is_inside(points, pixel) {
                put_pixel(pixel);
            }
        }
    }
}
```

어떠신가요? bounding box를 찾는 것은 매우 쉽습니다.
어느 한 점이 2D 삼각형에 속한지를 검사하는 것은 그리 어려운 문제가 아닙니다.

_polygon에 점이 속해있는지를 구현하고, plane에 대해 프로그램을 실행시키면, 원하는 값을 얻지 못할 것입니다.
이 문제를 안정적으로 해결하는 것은 매우 어려운 일입니다.
따라서, 여기서는 단순히 픽셀을 패인팅 할 것입니다. 이것으로 충분합니다._
_Off Topic: if I have to implement some code to check whether a point belongs to a polygon, and this program will run on a plane, I will never get on this plane. Turns out, it is a surprisingly difficult task to solve this problem reliably. But here we just painting pixels. I am okay with that._


 이 의사코드에서 맘에 드는 점이 하나 더 있습니다: 프로그래밍 개종자는 열정적으로 받아들이고, 조금 경력있는 프로그래머들은 종종 킥킥거립니다: "어떤 바보가 이걸 썼어".
컴퓨터 그래픽 프로그래밍 전문가는 그의 어깨를 의쓱하며 말하길: "이게 바로 실전이지".
수천개의 쓰래드에서 대규모 병렬계산은 사고 방식마져 바꾸게 만듭니다.
There is another thing I like about this pseudocode: a neophyte in programming accepts it with enthusiasm, more experienced programmers often chuckle: “What an idiot wrote it?”. And an expert in computer graphics programming will shrug his shoulders and say: “Well, that’s how it works in real life”. Massively parallel computations in thousands of threads (i’m talking about regular consumer computers here) change the way of thinking.


자, 시작해봅시다: 처음 우리가 알아야 할 것은 무게 중심 좌표계([barycentric coordinates](https://en.wikipedia.org/wiki/Barycentric_coordinate_system))입니다. 2D 삼각형 ABC와 점P가 주어지고, 데카르트좌표계(xy)에 있습니다.
저희 목표는 삼각형 ABC을 구성하는무게 중심 좌표계(barycentric coordinates)상의 점 P를 찾아내는 것입니다.
다음과 같은 3개의 수 (1 − u − v,u,v)로 P를 나타낼 수 있습니다:


![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index0x.png)


처음 볼때는 당황해 할지도 모르겠지만, 이는 매우 간단합니다:

버텍스A, B, C　각각에 (1 − u − v, u, v)라는 무게(weight)를 줍니다. 그러면 그에 해당하는 무게중심이 바로 점 P입니다.

바꿔말하자면: 점 P는 식 (A,![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index1x.png),![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index2x.png))상의 좌표 (u, v)입니다.
While being a bit frightening at the first glance, it is really simple: imagine that we put three weights (1 −u−v,u,v) at the vertices A, B and C, respectively. Then the barycenter of the system is exactly in the point P. We can say the same thing with other words: the point P has coordinates (u,v) in the (oblique) basis (A,![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index1x.png),![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index2x.png)):

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index3x.png)

벡터 ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index4x.png), ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index5x.npng) and ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index6x.png) 로, 두 실수 u, v에 대해 다음과 같이 정리할 수 있습니다:
So, we have vectors ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index4x.png), ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index5x.npng) and ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index6x.png), we need to find two real numbers u and v respecting the following constraint:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index7x.png)

이는 단순한 벡터 방정식이거나, 두 변수를 가진 두 선형 방정식입니다:
It is a simple vector equation, or a linear system of two equations with two variables:


![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index8x.png)

 선형방정식을 학구적으로 풀기에는, 저는 좀 게으른것같습니다. 메트릭스를 이용해 봅시다:
I am lazy and do not want to solve linear systems in a scholar way. Let us write it in matrix form:


![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/index9x.png)

이는, 저희가 찾고있는 벡터 (u, v, 1)이 (ABx,ACx,PAx) and (ABy,ACy,PAy)에 *동시에* 직교(orthogonal)한다는 것을 의미합니다. [이제 눈치를 체셨을 것이라고 봅니다](https://en.wikipedia.org/wiki/Cross_product). 약간의 힌트를 주자면: 평면상의 두 직선의 교차점을 찾으려면, 외적(cross product)를 계산하면 된다는 것입니다. 그럼 이제 테스트해봅시다: 주어진 두 점을 통과하는 방정식을 어떻게 찾을까요?
It means that we are looking for a vector (u,v,1) that is orthogonal to (ABx,ACx,PAx) and (ABy,ACy,PAy) *at the same time*! I hope you see [where I am heading](https://en.wikipedia.org/wiki/Cross_product). That is a small hint: to find an intersection of two straight lines in a plane (that is exactly what we did here), it is sufficient to compute one cross product. By the way, test yourself: how do we find an equation of a line passing through two given points?

자 그러면 이제, rasterization routine를 새로 짜봅시다:
주어진 삼각형의 bounding box안의 모든 pixel을 살펴볼 것입니다. 각 pixel에 대해 무게중심좌표를 계산합니다. 만일, 음수인게 있다면, 그 pixel은 삼각형 밖에 있다는 것입니다. 아마 코드를 바로 보는게 좀 더 빠를 것 같습니다:
So, let us program our new rasterization routine: we iterate through all pixels of a bounding box for a given triangle. For each pixel we compute its barycentric coordinates. If it has at least one negative component, then the pixel is outside of the triangle. Probably it is more clear to see the program directly:

```rust
use base::vec::{Vec2, Vec2i, Vec2f, Vec3f};
use std::cmp;


#[allow(non_snake_case)]
fn barycentric(pts: [Vec2i; 3], P: Vec2i) -> Vec3f {
    let p0 = pts[0];
    let p1 = pts[1];
    let p2 = pts[2];

    let u = Vec3f::new((p2.x - p0.x) as f32,
                       (p1.x - p0.x) as f32,
                       (p0.x - P.x) as f32)
            .cross(Vec3f::new((p2.y - p0.y) as f32,
                              (p1.y - p0.y) as f32,
                              (p0.y - P.y) as f32));

    if u.z.abs() < 1.0 {
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle(pts: [Vec2i; 3], image: &mut Image, color: &Color) {
    let clamp = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);

    let mut bboxmin = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);
    let mut bboxmax = Vec2i::new(0, 0);
    for i in 0..3 {

        bboxmin.x = cmp::max(0, cmp::min(bboxmin.x, pts[i].x));
        bboxmax.x = cmp::min(clamp.x, cmp::max(bboxmax.x, pts[i].x));

        bboxmin.y = cmp::max(0, cmp::min(bboxmin.y, pts[i].y));
        bboxmax.y = cmp::min(clamp.y, cmp::max(bboxmax.y, pts[i].y));
    }

    let mut P: Vec2i = Vec2i::new(0, 0);
    for x in bboxmin.x..bboxmax.x + 1 {
        for y in bboxmin.y..bboxmax.y + 1 {
            P.x = x;
            P.y = y;

            let bc_screen = barycentric(pts, P);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set_pixel(P.x as usize, P.y as usize, &color);
        }
    }
}

fn main() {
    let (width, height) = (200, 200);
    let mut image = Image::new(width, height);
    let pts = [Vec2i::new(10, 10), Vec2i::new(100, 30), Vec2i::new(190, 160)];
    triangle(pts, &mut image, &color::RED);
    image.write("out.tga").unwrap();
}

```

*barycentric()*함수는 저희가 이미 배운 방식으로, 주어진 삼각형 속 점 P의 좌표를 계산합니다.
그럼 이제  *triangle()* 함수가 어떻게 동작하는지 살펴봅시다.
우선, 좌하단과 우상단 2개의 점으로 정의된 bounding box를 계산합니다.

 Now let us see how works *triangle()* function. First of all, it computes a bounding box, it is described by two points: bottom left and upper right.

이 점들을 찾기 위해 삼각형의 버텍스를 돌며 min/max 좌표를 선택합니다.
 To find these corners we iterate through the vertices of the triangle and choose min/max coordinates.

저는 또한, 삼각형 밖에 삐저나오지 않도록 bounding box의 클리핑(clipping)하는 것을 추가하였습니다.
 축하합니다. 여러분. 이제 삼각형을 그리는 법을 알게 되었습니다!

I also added a clipping of the bounding box with the screen rectangle to spare the CPU time for the triangles outside of the screen. Congratulations, you know how to draw a triangle!


![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/0ba3f3e659f5feff80a78840fb927a71.png)


# Flat shading render

저희는 이미 속이 비어있는 삼각형으로 모델링 하는 방법을 알고 있습니다. 이제 색을 칠해봅시다. 얼마나 삼각형을 잘 체울 수 있는지 확인하기에 좋을 것입니다. 여기 코드가 있습니다:
We already know how to draw a model with empty triangles. Let us fill them with a random color. This will help us to see how well we have encoded filling of triangles. Here is the code:

```rust
for face in model.faces.iter() {
    let mut screen_coords: [Vec2i; 3] = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
    for j in 0..3 {
        let world_coords = model.vert(face[j]);
        screen_coords[j as usize] =
            Vec2i::new(((world_coords.x + 1.0) * width as f32 / 2.0) as i32,
                       ((world_coords.y + 1.0) * height as f32 / 2.0) as i32);
    }

    let r = rng.gen_range(0, 255) as u8;
    let g = rng.gen_range(0, 255) as u8;
    let b = rng.gen_range(0, 255) as u8;
    triangle(screen_coords, &mut image, &Color::new(r, g, b));
}
```

간단합니다: 예전에 했던것과 같이, 모든 삼각형을 돌면서, 월드 좌표를 스크린 좌표로 변환하고 삼각형을 그립니다.
다음 문서(article)에서 여러 좌표계를 다루도록 하겠습니다.
그림은 다음과 같을 것입니다:
It is simple: just like before, we iterate through all the triangles, convert world coordinates to screen ones and draw triangles. I will provide the detailed description of various coordinate systems in my following articles. Current picture looks something like this:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/0c58d8a735.png)


이제 얼룩달룩한 색 대신, 빛을 넣어봅시다.
 Captain Obvious: "빛의 밝기가 동일하다면, 빛의 방향과 직교일때 폴리곤이 가장 밝게 빛난다."
Let us get rid of these clown-colors and put some lighting. Captain Obvious: ”At the same light intensity, the polygon is illuminated most brightly when it is orthogonal to the light direction.”

비교해 보도록 합시다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/5371a416d1.jpg)

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/97e210ee08.jpg)

폴리곤이 빛의 벡터와 평행하다면 얻게되는 illumination은 zero입니다.
바꿔말하자면:
빛의 밝기는 빛의 벡터와 주어진 삼각형의 노말의 스칼라곱과 같다.
삼각형에 대한 노말값은 두 변의 [외적(cross product)](https://en.wikipedia.org/wiki/Cross_product)으로 간단히 계산할 수 있다.

We get zero illumination if the polygon is parallel to the vector of light. To paraphrase: the intensity of illumination is equal to the scalar product of the light vector and the normal to the given triangle. The normal to the triangle can be calculated simply as the [외적(cross product)](https://en.wikipedia.org/wiki/Cross_product) of its two sides.

덧붙여서, 이번 강좌에서 저희는 색에 대한 선형 계산을 할 것입니다.
하지만 색상 (128, 128, 128)은 (255, 255, 255)의 절반에 해당하는 밝기를 가진건 아닙니다.
감마의 정확성과 빛의 밝기가 부정확한 것은 일단 놔두고 진행하도록 하겠습니다.
As a side note, at this course we will perform linear computations on the colors. However (128,128,128) color is not half as bright as (255, 255, 255). We are going to ignore gamma correction and tolerate the incorrectness of the brightness of our colors.

```rust
let light_dir = Vec3f::new(0.0, 0.0, -1.0);
for face in model.faces.iter() {
    let mut screen_coords: [Vec2i; 3] = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
    let mut world_coords: [Vec3f; 3] =
        [Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0)];

    for j in 0..3 {
        let v = model.vert(face[j]);
        screen_coords[j as usize] = Vec2i::new(((v.x + 1.0) * width as f32 / 2.0) as i32,
                                               ((v.y + 1.0) * height as f32 / 2.0) as i32);
        world_coords[j as usize] = v;
    }

    let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]);
    let intensity = light_dir * n.normalized();
    if intensity > 0.0 {
        let c = (intensity * 255.0) as u8;
        triangle(screen_coords, &mut image, &Color::new(c, c, c));
    }
}
```

외적(dot product)는 음수가 될 수 있습니다. 이는 뭘 의미할까요? 바로 빛이 폴리곤 뒤에서 비친다는 것입니다.
씬이 잘 구성되었다면, 이러한 삼각형을 간단한하게 무시할 수 있을것입니다.
보이지 않는 삼각형들을 빠르게 없앨 수 있는 방법이 있습니다. 바로 [Back-face culling](http://en.wikipedia.org/wiki/Back-face_culling)입니다.
But the dot product can be negative. What does it mean? It means that the light comes from behind the polygon. If the scene is well modelled (it is usually the case), we can simply discard this triangle. This allows us to quickly remove some invisible triangles. It is called [Back-face culling](http://en.wikipedia.org/wiki/Back-face_culling).

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/02-triangle/d5223f9b93.png)

입 안쪽 구강이 입술 위에 그려진 것을 주목해주시기 바랍니다. 이는 보이지않는 삼각형들의 클리핑이 잘못되었기 때문입니다: 볼록한 모양일때만 제대로 동작합니다. 다음시간에 z-buffer를 인코드하여 이것들을 없에보도록 하겠습니다.
Note that the inner cavity of the mouth is drawn on top of the lips. That is because of our dirty clipping of invisible triangles: it works perfectly for convex shapes only. We will get rid of this artifact next time when we encode the z-buffer.

[Here’s](https://github.com/ssloy/tinyrenderer/tree/e1a3f2b0f9638fa6db9e0437c621132e1baa3fb1) the current version of the render.
