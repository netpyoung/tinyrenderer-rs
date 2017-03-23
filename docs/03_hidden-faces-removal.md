# Introduction
제 흑인 친구녀석 z-buffer를 소개합니다.[TODO(kep)]

Hello, let me introduce you my friend z-buffer of a black guy. He will help us get rid of the visual artifacts of the hidden faces removal we had during the last lesson.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/3f057a75601d8ac34555e72ea03ef711.png)

어쨌거나, 제 강좌에서 다루는 이 모델은
[Vidar Rapp](https://se.linkedin.com/in/vidarrapp)이 만들었습니다. 그는 친절하게도 저에게 저작권을 보장해주었습니다 렌더링 기초를 가르치는데 사용하도록
제가 좀 망치긴 했지만, 곧 눈을 돌려주도록 하겠습니다.
By the way, i'd like to mention that this model i use heavily in the course is created by [Vidar Rapp](https://se.linkedin.com/in/vidarrapp). He kindely granted me a permission to use it for teaching rendering basics and i vandalized it, but i promise you to give back the eyes to the guy.


다시 주제로 돌아와서, 이론적으로 버리는것 하나없이 모든 삼각형을 그릴 수 있습니다
적절히 멀리있는 것부터 그리다보면 가까이에 있는 것을 그릴때 전에 그린게 지워질 것입니다.
Well, back to the topic, in theory we could just draw all the triangles without discarding any. If we do it properly starting rear-to-front, the front facets will erase the back ones.

이를 [painter's algorithm](http://en.wikipedia.org/wiki/Painter%27s_algorithm)라고 부릅니다.
 It is called the [painter's algorithm](http://en.wikipedia.org/wiki/Painter%27s_algorithm).
불행히도, 많은 계산 비용을 요구합니다:
각각의 카메라 이동에 대해 모든 씬에 대해 재정렬이 필요합니다.
 Unfortunately, it comes along with a high computational cost: for each camera movement we need to re-sort all the scene.

동적 씬이 존재하기도 하며... 하지만 이건 중요한 문제가 아닙니다. 중요한 문제는 정확한 순서를 알 수 없다는 것입니다.
 And then there are dynamic scenes... And this is not even the main problem. The main problem is it is not always possible to determine the correct order.



# 간단한 씬을 랜더링 해봅시다.
삼각형으로 이루어진 간단한 씬을 상상해봅시다: 카메라는 위에서-아래로 향하고있고, 색깔있는 삼각형을 흰 스크린에 투영하도록 합니다:
Imagine a simple scene made of three triangles: the camera looks up-to-down, we project the colored triangles onto the white screen:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/d493c52da4cabe9a057c26f696784956.png)

다음과 같이 렌더링 될것입니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/023668cb8ea97f59bf87d982c1e8b030.png)

파란면은 빨강면의 뒤에 있을까요? 앞에있을까요? painter's algorithm 은 여기서 먹히지 않습니다.
Blue facet - is it behind or in front of the red one? The painter's algorithm does not work here.

파란면을 2개로 나누고 나서야 가능합니다(하나는 빨강면의 앞에, 다른 하나는 뒤에).
 It is possible to split blue facet in two (one in front of the red facet and one behind).

빨강면 앞에 있는 것을 2개로 나눕니다 - 녹색 삼각형 앞에 있는 것과 뒤에 있는것..
 And then the one in front of the red one is to be split in two - one in front of the green triangle and one behind...


문제가 있습니다: 씬안에 수백만의 삼각형이 있으면 계산하는데 매우 부하가 클것입니다.
I think you get the problem: in scenes with millions of triangles it is really expensive to compute.

이러한 부하는 [BSP trees](https://en.wikipedia.org/wiki/Binary_space_partitioning)로 줄일 수 있습니다.
 It is possible to use [BSP trees](https://en.wikipedia.org/wiki/Binary_space_partitioning) to get it done.
어쨋거나, 이러한 데이터 구조는 움직이는 카메라에 대해 고정적이나, 매우 지저분합니다.
By the way, this data structure is constant for moving camera, but it is really messy.

 지저분하게 하기에는 삶이 너무나 짧습니다
 And the life is too short to get it messy.


더 간단하게: 차원을 줄이자. Y-buffer!
# Even simpler: let us lose a dimension. Y-buffer!

차원을 줄이기 전에 노란 평면으로 위의 씬을 자르도록 하겠습니다:
Let us lose a dimension for a while and to cut the above scene along the yellow plane:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/d673f40bcadbe53f4b3cb29bbbcfb461.png)

제가 의도한대로, 저희 씬은 이제 3개의 선으로 이루어졌으며, 최종적으로 렌더링 화면 넓이는 그대로지만, 높이는 1픽셀로 되었습니다:

I mean, now our scene is made of three line segments (intersection of the yellow plane and each of the triangles),
and the final render has a normal width but 1 pixel height:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/3d4c4a1710b8e2558beb5c72ea52a61a.png)



As always, there is a [commit](https://github.com/ssloy/tinyrenderer/tree/d9c4b14c0d8c385937bc87cee1178f1e42966b7c) available.


저희 씬은 2차원이므로, line() 함수를 이용하여 쉽게 그릴 수 있습니다.
Our scene is two-dimensional, so it is easy to draw it using the line() function we programmed in the very first lesson.

```rust
let (width, height) = (800, 500);

let mut image = Image::new(width, height);

line(&Vec2i::new(20, 34), &Vec2i::new(744, 400), &mut image, &color::RED);
line(&Vec2i::new(120, 434), &Vec2i::new(444, 400), &mut image, &color::GREEN);
line(&Vec2i::new(330, 463), &Vec2i::new(594, 200), &mut image, &color::BLUE);
line(&Vec2i::new(10, 10), &Vec2i::new(790, 10), &mut image, &color::WHITE);

image.write("out.tga").unwrap();
```


측면에서 바라보면 다음과같은 2D 씬처럼 보일것입니다:
This is how our 2D scene looks like if we look at it sideways:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/20e9d8742d17979ec70e45cafacd63a5.png)

이제 렌더링을 해봅시다.
renderer는 1 pixel 높이라는 것을 기억합니다.
Let us render it. Recall that the render is 1 pixel height.

높은 해상도의 화면에서도 읽기 쉽도록 저는 16 pixel 높이를 만들 것입니다.

 In my source code I create images 16 pixels height for the ease of reading on high resolution screens.

 *rasterize()* 함수는 이미지 *render*의 오직 첫번째 선만 씁니다.
 *rasterize()* function writes only in the first line of the image *render*



```rust
const WIDTH: usize = 800;
let height = 16;

let mut render = Image::new(WIDTH, height);
let mut ybuffer = [i32::min_value(); WIDTH];
rasterize(&Vec2i::new(20, 34),   &Vec2i::new(744, 400), &mut render, &color::RED,   &mut ybuffer);
rasterize(&Vec2i::new(120, 434), &Vec2i::new(444, 400), &mut render, &color::GREEN, &mut ybuffer);
rasterize(&Vec2i::new(330, 463), &Vec2i::new(594, 200), &mut render, &color::BLUE,  &mut ybuffer);

// 1-pixel wide image is bad for eyes, lets widen it
for i in 0..WIDTH {
    for j in 1..16 {
        let color = render.get_pixel(i, 0);
        render.set_pixel(i, j, &color);
    }
}
```

또한, 저는 *(width, 1)* 차원의 *ybuffer* 배열을 선언하였습니다.
So, i declared a magic array *ybuffer* with dimensions *(width, 1)*.

이 배열은 음의 무한대로 초기화되었습니다.
 This array is initialized with minus infinity.

그런다음 앞서 선언한 배열과 이미지 *render*를 인자로 *rasterize()* 를 호출하였습니다.
 Then i call *rasterize()* function with this array and the image *render* as arguments.

어떤 모양이 나타날까요?
 How does the function look like?

```rust
const WIDTH: usize = 800;

fn rasterize(p0: &Vec2i, p1: &Vec2i, image: &mut Image, color: &Color, ybuffer: &mut [i32: WIDTH]) {
    let mut p0 = Vec2i::new(p0.x, p0.y);
    let mut p1 = Vec2i::new(p1.x, p1.y);

    if p0.x > p1.x {
        mem::swap(&mut p0, &mut p1);
    }

    for x in p0.x..p1.x {
        let t = (x - p0.x) as f32 / (p1.x - p0.x) as f32 ;
        let y = (p0.y as f32 * (1.0 - t) + p1.y as f32 * t) as i32;

        if ybuffer[x as usize] < y {
            ybuffer[x as usize] = y;
            image.set_pixel(x as usize, 0usize, color);
        }
    }
}
```


```
// 역주. 흰선이 아레서부터 위로 올라가면서 마주치는 부분을 색칠 및 ybuffer를 동시에 채워나가고 있다고 상상!!
// ![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/20e9d8742d17979ec70e45cafacd63a5.png)
```



매우 매우 간단합니다: p0.x와 p1.x사이의 x좌표를 돌면서 그에 대응하는 y좌표를 계산하였습니다.
It is really-really simple: i iterate through all x-coordinates between p0.x and p1.x and compute the corresponding y-coordinate of the segment.

현재 x를 인덱스로 *ybuffer* 배열을 확인하였습니다.
 Then i check what we got in our array *ybuffer* with current x index.

현재 y값이 ＊ybuffer*에 들어있는 값보다 카메라에 가깝다면, 화면에 그리고 *ybuffer*를 갱신합니다.
 If the current y-value is closer to the camera than the value in the *ybuffer*, then i draw it on the screen and update the *ybuffer*.

이제 찬찬히 살펴봅시다.
*rasterize()*를 호출하면 다음과 같은 (빨간)선이 메모리에 올라가게 됩니다:
Let us see it step-by-step. After calling *rasterize()* on the first (red) segment this is our memory:

screen:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/01694d604755b68c406998c03db374d9.png)

ybuffer:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/65ddaf2b4d87f9b80127ecc6b02d0f72.png)

마젠다 색상은 음의 무한대를 나타내며, 화면상 건드리지 않은 부분과 일치합니다.
Here the magenta color indicates the minus infinity, those are places corresponding to the screen we did not touch.

나머지 부분은 회색으로 그림자가 꼈습니다: 카메라에 가까울 수록 색이 밝아지고, 멀어질수록 어두워집니다.
 All the rest is shown in the shades of gray: clear colors are close to the camera, dark colors far from the camera.


그 다음 녹색선을 그립니다.
Then we draw the green segment.

screen:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/6f081ac5fc77e2ec4bc733c945b16615.png)

ybuffer:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/bae97132fc4ae67584b46b03d7350944.png)

마지막으로 파란선을 그립니다.
And finally the blue one.

screen:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/d6fdb1d49161923ac91796967afa766e.png)

ybuffer:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/8f430d7de76bdcbda73b8de2986fbe49.png)


훌륭합니다. 2D 씬을 1D 씬에 그렸습니다!  이 얼마나 경외롭습니까:
Congratulations, we just drew a 2D scene on a 1D screen! Let us admire once again the render:
![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/24935d71a1b0023ee3cb48934fae175d.png)


# 3D로 회귀

자, 2D 화면에 그리기 위해서는 z-buffer는 2차원이 되어야 합니다:
So, for drawing on a 2D screen the z-buffer must be two-dimensional:

```rust
let mut zbuffer = [i32::min_value(); WIDTH * HEIGHT];
```

개인적으로 저는 2차원 버퍼를 1차원으로 표현하였으며, 전환과정은 간단합니다:
Personally i pack a two-dimensional buffer into a one-dimensional, the conversion is trivial:

```rust
let idx = x + y * WIDTH;
```

되돌릴려면:

```rust
let x = idx % WIDTH;
let y = idx / WIDTH;
```

코드에서 모든 삼각형을 살펴보면서 해당 삼각형과 z-buffer를 인자로 rasterizer 함수를 호출할겁니다.
Then in the code i simply iterate through all the triangles and call the rasterizer function with current triangle and a reference to the z-buffer.

차이점이 있다면, 그리고자 하는 pixel의 z-value를 계산하는 방식일 것입니다.
The only difficulty is how to compute the z-value of a pixel we want to draw.

y-buffer에서 y값을 계산했던 것을 다시 떠올려봅시다:
Let us recall how we computed the y-value in the y-buffer example:

```rust
let y = (p0.y * (1.0 - t) + p1.y * t) as i32;
```



변수 *t* 의 본래 값은 무엇일까요?
What is the nature of the *t* variable?


*(1-t, t)*는 `(x, y) = p0 * (1 - t) + p1 * t` 선분 p0, p1에 대한 점 (x, y)의 무게중심좌표입니다.
 It turns out that *(1-t, t)* are barycentric coordinates of the point (x,y) with respect to the segment p0, p1:  (x,y) = p0\*(1-t) + p1\*t.


삼각형 rasterization의 무게중심좌표 버전에서 아이디어를 취해, 그리고자 하는 모든 픽셀의
무게중심좌표와 삼각형의 버텍스의 z값을 단순히 곱합니다:
So the idea is to take the barycentric coordinates version of triangle rasterization, and for every pixel we want to draw simply to multiply its barycentric coordinates by the z-values of the vertices of the triangle we rasterize:


```rust
triangle(pts, &mut zbuffer, &mut render, &Color::new(c, c, c));

[...]

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle(pts: [Vec3f; 3], zbuffer: &mut [f32], image: &mut Image, color: &Color) {
    let clamp = Vec2f::new(image.width as f32 - 1.0, image.height as f32 - 1.0);

    let mut bboxmin = Vec2f::new(f32::MAX, f32::MAX);
    let mut bboxmax = Vec2f::new(f32::MIN, f32::MIN);
    for i in 0..3 {
        bboxmin.x = f32::max(0.0, f32::min(bboxmin.x, pts[i].x));
        bboxmax.x = f32::min(clamp.x, f32::max(bboxmax.x, pts[i].x));

        bboxmin.y = f32::max(0.0, f32::min(bboxmin.y, pts[i].y));
        bboxmax.y = f32::min(clamp.y, f32::max(bboxmax.y, pts[i].y));
    }

    let mut P = Vec3f::new(0.0, 0.0, 0.0);
    for x in bboxmin.x as i32 ..bboxmax.x as i32+ 1 {
        for y in bboxmin.y as i32 ..bboxmax.y as i32 + 1 {
            P.x = x as f32;
            P.y = y as f32;

            let bc_screen = barycentric(pts, P);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            P.z = 0.0;
            P.z += pts[0].z * bc_screen.x;
            P.z += pts[1].z * bc_screen.y;
            P.z += pts[2].z * bc_screen.z;

            let idx = (P.x + P.y * image.width as f32) as usize;
            if zbuffer[idx] < P.z {
                zbuffer[idx] = P.z;
                image.set_pixel(P.x as usize, P.y as usize, &color);
            }

        }
    }
}
```


이 얼마나 대단합니까. 약간의 코드 수정으로 이전 강의에서 숨겨진 부분들을 없앨 수 있다는 것이.
여기 렌더된 결과물이 있습니다:
It is terrific how little changes we made to the source code from the previous lesson to discard the hidden parts!
Here is the render:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/f93a1fc1cbaebb9c4670ae0003e62947.png)

The source code can be found [here](https://github.com/ssloy/tinyrenderer/tree/68a5ae382135d679891423fb5285fdd582ca389d).

# 자, z값을 합쳤고, 이제 무엇을 해야할까요?
# Okay, we just interpolated the z-values. What else can we do?

텍스쳐! 숙제를 내도록 하겠습니다.
Texture! It would be our home assignment.

.obj파일에는 "vt u v"로 시작하는 줄이 있으며, 이는 텍스쳐 좌표의 배열을 나타냅니다.
In the .obj file we have lines starting with "vt u v", they give an array of texture coordinates.

슬러쉬간 중간에 있는 숫자는 삼각형의 버텍스의 텍스쳐 좌표입니다
The number in the middle (between the slashes) in the facet lines "f x/x/x x/x/x x/x/x" are the texture coordinates of this vertex of this triangle.

삼각형과 합쳐, 텍스쳐 이미지의 높이-곱을 곱하면 렌더할 결과물에 집어놓을 색상을 얻을 수 있을 것입니다.
 Interpolate it inside the triangle, multiply by the width-height of the texture image and you will get the color to put in your render.

디퓨즈 텍스쳐는 [여기서](https://github.com/ssloy/tinyrenderer/raw/master/obj/african_head/african_head_diffuse.tga) 얻을 수 있습니다.
Diffuse texture can be taken [here](https://github.com/ssloy/tinyrenderer/raw/master/obj/african_head/african_head_diffuse.tga).


Here is an example of what I expect from you:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/03-zbuffer/73714966ad4a4377b8c4df60bef03777.png)
