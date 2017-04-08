``https://github.com/sebcrozet/nalgebra

# 목표

이전 강좌에서는 z좌표를 빼먹은채 직교투영(orthographic projection)으로 모델을 렌더링하였습니다. 오늘은 원근투형(perspective)으로 그려보도록 하겠습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/39467dda61fdb644e68bdafc1e1f17f1.png)


# 2D 기하학(geometry)

## 선형변환(Linear transformations)

평면에서의 선형변형(linear transformation)은 매트릭스로 표현할 수 있습니다. 점(x, y)이 있으면, 변환(transformation)은 다음과 같이 쓸 수 있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f00.png)


가장 간단한 변환(transformation)은 단위행렬(identity)이며, 이는 어떠한 점으로도 이동하지 않습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f01.png)


매트릭스의 대각계수(Diagonal coefficients)는 크기조절(scaling)을 담당합니다. 다음 변환을 취했다고 한번 상상해봅시다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f02.png)


한쪽 모서리가 접힌 사각형 하얀색 물체가 노란색으로 변형될 것입니다. 빨강선과 녹색선 각각은 x, y를 나타내는 유닛 벡터입니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/2aa8b671e124f1511c3b47a37c47f150.png)


왜 아까부터 자꾸 매트릭스를 들먹일까요? 매우 유용하기 때문입니다. 우선, 매트릭스로 다음과 같이 전체 오브젝트의 변환(transformation)을 표현할 수 있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f03.png)


이 표현식에서의 변환 매트릭스는 이전것과 같지만, 2x5매트릭스는 이전과는 다른 사각형 물체의 정점들입니다. 배열에 있는 모든 버텍스들을 변환 매트릭스랑 곱하면, 변환된 오브젝트를 얻을 수 있습니다. 쩔지 않습니까? 이제　저희는 오브젝트를 자꾸 바꿔보도록 할것입니다.　다음과 같이 변환함수를 작성한다고 가정해봅시다.

```rust
fn foo(p: vec2) -> vec2 {
    vec2(ax + by, cx + dy)
}

fn bar(p: vec2) -> vec2 {
    vec2(ex + fy, gx + hy)
}

[..]

for p in object.into_iter() {
    let p = foo(bar(p));
}
```

이 코드는 오브젝트의 정점 각각에 대해 두번의 선형변환을 수행하며, 저희는 이러한 정점을 수만번이나 만나게 될 것입니다. 몇십번 변환하는건 예삿일이 아니며, 수천만번 변환하게 될 것이며, 이는 계산비용이 많이 드는 작업이 될 것입니다. 저희는 매트릭스 성질을 이용하여, 변환 매트릭스 모두 미리 곱해, 오브젝트를 한번에 변환 할 수 있습니다.　곱셈 계산 식에서 괄호를 어디에 넣든 상관 없으니까요.　좋습니다. 계속해봅시다.　매트릭스의 대각계수는 좌표계를 늘린다는 것을 알 수 있습니다.　다른 계수는 어디에 쓸까요? 다음과 같은 변환이 있다고 가정해봅시다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f04.png)

여기 적용된 오브젝트가 있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/bb13159ffc0656ee622f9c4ebd108fed.png)

x-축으로 기울었습니다. 대각 반대쪽의 요소는 y-축으로 기울게 할것입니다. 두개의 선형 변환 크기조절(scaling)과 전단(shearing)이 있습니다. 잠깐 회전(rotation)은 어떻게?　회전(rotation)은 3번의 전단(shear)을 조합으로 표현할 수 있으며, 여기 흰색 오브젝트가 빨간색으로 변형 후, 녹색으로 마지막으로 파란색으로 변형되는 과정이 나와있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/8723ca291b463b6eb44b9a91f5cbd26f.png)

하지만, 좀 복잡하니까, 간단히해서 회전 매트릭스를 곧바로 쓸 수 있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f05.png)

이 매트릭스를 어느 방향으로 곱할 순 있지만, 매트릭스의 곱은 교환법칙이 성립하지 않습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f06.png)

오브젝트를 전단(shear) 후 회전(rotate)하는 것은, 회전 후 전단하는 것과 같지않습니다!

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/7a85ee0ebed76be99ba9f97f0c89c5a4.png)


# 2D 아핀변환(affine transformations)

따라서, 평면 위 선형 변환은 크기조절(scale)과 전단(shear) 변환의 조합입니다. 이는, 원점을 이동시키지 않으면서, 원하는 선형 변환을 할 수 있다는 것을 의미합니다! 가능성 자체는 좋지만, 변환하는 방법이 어렵다면 실로 비참할 것입니다. 할 수 있을까요?, 좋습니다,　이동은 선형이 아니긴 하지만, 문제없습니다. 선형 부분을 수행 후에 이동을 넣어보도록 하겠습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f07.png)

위의 표현식은 매우 훌륭합니다. 회전(rotate), 크기변형(scale), 전단(shear) 그리고 이동(translate)을 할 수 있습니다. 하지만. 여러 변형을 조합할 수 있다는 것을 상정할때, 두개의 변형을 합치면 다음과 같아질 것입니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f08.png)

한번 합치는데도 이렇게 복잡한데, 여러번 합치면 더 심해질 것입니다.


# 동차좌표(Homogeneous coordinates)

좋습니다. 이제 흑마법의 시간입니다. 변형 매트릭스에 열과 행을 하나씩 추가하여 3x3 매트릭스를 만들고, 변형이 잘 이뤄지도록 좌표 하나를 1로 넣어 봅시다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f09.png)

이 매트릭스와 1이 들어있는 벡터와 곱하게 되면, 놀랍게도 저희가 저희가 원하는 모양이 나오게됩니다!　사실, 이 아이디어는 매우 간단합니다. 2D공간에서 평행 이동은 선형적이지가 않습니다. 따라서, 3D공간에 2D를 내포하도록 만들었습니다. 이는 평면 z = 1인 3D공간에 2D공간이 있다는 것을 의미합니다. 다음에 선형 3D 변형을 수행하여, 결과를 2D 평면에 투영시킵니다. 평행 이동은 선형적일 수 없으나, 과정자체는 매우 단순합니다. 그렇다면, 3D를 어떻게 2D로 되돌릴까요? 간단하게 3번째 요소로 나누면 됩니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f10.png)



## 잠깐, 0으로 나누면 안된다고!

지금 말한건 누굽니까? 과정을 다시 살펴봅시다:
* 2D를 평면 z = 1인 3D에 내장시켰습니다.
* 3D상에서 원하는 것을 처리합니다.
* 3D를 2D로 투영시키고자 TODO
* For every point we want to project from 3D into 2D we draw a straight line between the origin and the point to project and then we find its intersection with the plane z=1.

점 (x, y, z)가, 마젠타색상의 2D 평면 위의 (x / z, y / z)로 투영되었습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/47cf05bf642df13f9b738e2c3040f648.png)

점(x, y, 1)에 수선을 그어봅시다. 점(x, y, 1)이 어디에 투영될까요? 바로 (x, y)입니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/0c054967a27e66bf020844118a1750d8.png)

수선을 내려가보면, 점(x, y, 1/2)는 점(2x, 2y)에 투영됩니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/ed24b22a0542f9f930e0386c598d5a77.png)

계속 내려가보면, 점(x, y, 1/4)는 (4x, 4y)가 됩니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/9e9658d91a6c8198606a8603012f048a.png)

z = 0에 가까워 질 수록, 투영 결과는 원점 (x, y)보다 멀리 떨어지게 될 것입니다.
If we continue the process, approaching to z=0, then the projection goes farther from the origin in the direction (x,y).

다시말해, 점(x, y, 0)는 (x, y) 방향으로 무한대로 투영된다는 것입니다.
 In other words, point (x,y,0) is projected onto an infinitely far point in the direction (x,y).

이것은 무얼 의미하는 걸까요? 그렇습니다. 바로 벡터입니다!
 What is it? Right, it is simply a vector!

이러한 동차좌표(Homogeneous coordinates)는 벡터와 점을 구분 할 수 있도록 해 줍니다.

 allow to distinguish between a vector and a point.

vec2(x, y)는 벡터일까요 포인터일까요? 바로 말하기 어렵습니다. z = 0인 동차좌표라면 벡터일 것이고, 아니면 점일것입니다.
 If a programmer writes vec2(x,y), is it a vector or a point? Hard to say. In homogeneous coordinates all things with z=0 are vectors, all the rest are points.
 vector + vector = vector. Vector - vector = vector. Point + vector = point.


## 조합 변형(A composite transformation)

앞서 말한것처럼, 여러 변형을 묶을 수 있어야 합니다. 점(x0, y0)를 중점으로 오브젝트를 회전하고자 합니다. 어떻게 할까요? 공식을 살펴볼 수 도 있고, 손으로 계산해도 됩니다.

중점을 중심으로 회전하는 방법을 알고, 움직이는 방법을 알고 있습니다.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f11.png)

3D에서는 해야할 것이 조금 길어질 뿐이지, 아이디어는 같습니다.


# 잠깐만, 3x3매트릭스의 마지막 줄을 바꿔도 되나요?

물론이죠! 다음 변형을 적용시켜 봅시다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f12.png)

원본 오브젝트는 흰색에, 단위 벡터는 빨간색과 녹색으로 나타냈습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/7f36ab01dad4a2937599de236c8d4d28.png)

여기 변형된 오브젝트가 있습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/ff8f6a2130986fed747e55a26e054c6f.png)


이건 또 다른 종류의 마법입니다(백마법). y-버퍼를 기억하시나요?
And here another kind of magic (white!) happens. Do you remember our y-buffer exercise?

여기서도 동일한 일을 하였습니다: 2D 오브젝트를 x = 0인 수직선에 투영시켰습니다.
 Here we will do the same: we project our 2D object onto the vertical line x=0.

좀 더 어려운 문제로 가봅시다:
중심투영법(central projection)을 활용해 보기 위해,
중점을 바라보고 있는 카메라가 점 (5, 0)에 위치해 있다고 가정해 봅시다.

 Let us harden the rules a bit: we have to use a central projection, our camera is in the point (5,0) and is pointed onto the origin.

투영된 것을 얻기 위해선 카메라와 투영된 점들을 선으로 이어(노란선), 화면과 교차된 것(하얀색 세로선)을 찾아야 합니다.
 To find the projection we need to trace straight lines between the camera and the points to be projected (yellow) and to find the intersection with the screen line (white vertical).

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/a7081e13ad5016aa33f87edb50b218f0.png)


이제 노란선을 놔둔 체, 원래 오브젝트를 변형시켜 보도록 하겠습니다:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/2b9f233797ca0a8b2d9d9f9750c29a36.png)

**표준 정사영(standard orthogonal projection)**을 활용하여 빨간 오브젝트를 화면에 투영한다면, 동일한 점들을 얻을 수 있을 것입니다!
If we project the red object onto the screen using **표준 정사영(standard orthogonal projection)**, then we find exactly the same points!

이제 변형이 어떻게 동작하는지 자세히 살펴봅시다: 수직선들은 카메라와 가까이 있는 것들은 늘어나게되며, 멀리있는 것들은 줄어들게 됩니다.
 Let us look closely how the transformation works: all  vertical segments are transformed into vertical segments, but those close to the camera are stretched and those far from the camera are shrunk.

변형매트릭스에서 계수가 -1/5지만, 적절한 계수를 고른다면 정사형된 이미지를 얻을 수 있을 것입니다!



# full 3D로 작업할 시간(Time to work in full 3D)
마법을 까발려봅시다. 2D 아핀변환과 같이, 3D 아핀변환으로 homogeneous coordinates를 이용할 것입니다: 점(x, y, z, 1)로 4D에 있는 걸 다시 3D로 투영시켜 변환할 것입니다. 예를들어, 다음과 같은 formation을 가지고 있다고 가정해봅시다:
Let us explain the magic. As for 2D affine transformations, for 3D affine transformations we will use homogeneous coordinates: a point (x,y,z) is augmented with 1 (x,y,z,1), then we transform it in 4D and project back to 3D. For example, if we take the following transformation:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f13.png)

고전적인-투영은 다음과 같은 3D 좌표로 나타낼 수 있습니다:
The retro-projection gives us the following 3D coordinages:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f14.png)

결과를 기억해둡시다.
4D transformation과 같은 멋진것이 없은
다시 centeral projection의 표준정의로 돌아옵시다.
점 P=(x, y, z) 가 주어졌고
z-축에 점 (0, 0, c)에 위치한 camera 로 plane z = 0으로 투영시키고자 합니다.
Let us remember this result, but put it aside for a while. Let us return to the standard definition of the central projection, without any fancy stuff as 4D transformations. Given a point P=(x,y,z) we want to project it onto the plane z=0, the camera is on the z-axis in the point (0,0,c):

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/525d3930435c3be900e4c7956edb5a1c.png)

삼각형 ABC과 ODC는 유사합니다.
이는 다음과 같이 쓸 수 있습니다: |AB| / |AC| = |OD| / |OC| => x / (c - z) = x' / c. 다시 말해:
Triangles ABC and ODC are similar. It means that we can write the following: |AB|/|AC|=|OD|/|OC| => x/(c-z) = x'/c. In other words:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f15.png)

동일한 이유로 삼각형 CPB와 CP'D를 다음과 같은 표현식으로 나타낼 수 있습니다:
By doing the same reasoning for triangles CPB and CP'D, it is easy to find the following expression:

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f16.png)

이는 아까전에 봤던 결과와 매우 유사합니다------------.
It is really similar to the result we put aside few moments ago, but there we got the result by a single matrix multiplication. We got the law for the coefficient: r = -1/c.


# Let us sum up: the main formula for today

*이 공식을 이해하지 않고 단순히 복사-붙여넣기하면, 널 반드시 찾아내서...*
*If you simply copy-paste this formula without understanding the above material, I hate you.*


So, if we want to compute a central projection with a camera **(important!) camera located on the z-axis with distance c from the origin**, then we embed the point into 4D by augmenting it with 1, then we multiply it with the following matrix, and retro-project it into 3D.

![](https://raw.githubusercontent.com/ssloy/tinyrenderer/gh-pages/img/04-perspective-projection/f17.png)

We deformed our object in a way, that simply forgetting its z-coordinate we will get a drawing in a perspective. If we want to use the z-buffer, then, naturally, do not forget the z. The code is available [here](https://github.com/ssloy/tinyrenderer/tree/1cce85258d1f1cf75fd10fe4d62ebfdb669f8cf9), its result is visible in the very beginning of the article.
