ref: http://www.poshy.net/1405
ref: https://web.cs.wpi.edu/~emmanuel/courses/cs543/f13/slides/lecture10_p3.pdf

선을 어떻게 그릴까
격자모양에 점과 점사이를 어떻게 채워넣을까

# simple line drawing
xy좌표에서 y에 대한 선분f(x) = y라 할때 m : 기울기, n : y 절편이라고 하면 다음과 같은 식이 나타난다.

f(x) = y = (m * x) + n
m = dy / dx = (y1 - y0)/(x1 - x0)

좌표상에 점을 그리기 위해 보통 좌에서 우로 그리는 x축 loop를 돌린다.
따라서 x축으로 긴 선을 그릴때는 잘 그려지지만, y축으로 긴 선을 그릴때에는, (dx와 dy의 차이때문에)선 중간중간 틈이 보이게 된다.



# DDA(Digital Differential Analyzer) line drawing
## Improved Simple Line drawing Algorithm
simple line drawing의 틈을 매꾸기 위해, dx와 dy를 비교하여 x축 루프, y축 루프를 돌릴지 판단한다.


# Bresenham
Improved Simple Line drawing Algorithm에서 불필요한 연산 제거.

* 일반적으로 덧셈과 뺄셈은 곱셈보다 빠르다.
* 일반적으로 곱셈을 나눗셈보다 빠르다.
* 속도개선을 위해서 Look Up Table을 사용할수 있다.(산재되어 있는 값들의 테이블을 이용할수 있다.)
* 조건문에서 본질적으로 값들이 판단하는것은 임의의 숫자가 아닌 0 값과 비교하는것이다.(a > b 는 a-b >0 으로 해석되어 진다.)
* incremental algorithm : 이전 값을 이용하여 현재값을 구함
* integer only(Float형보다 Int형 계산이 더 빠르다.)
* variant : round => mid-point ...

m = dy / dx = (y1 - y0) / (x1 - x0)
n = y - (m * x) = y0 - (m * x0)

알고리즘을 구현하기 위해서는 우선 증가되는 기울기 값에 대해 그 값이 어느 좌표에 가까운가를 판단하는 부분이 필요.
mid-point를 설정해서, 그것을 판단.

mid-point M(Mx, My) = (x + 1, y + 1/2)
        (x + 1, y + 1)
        (x + 1, y + 1/2)를 기준으로 위로 점을 찍을지 아래로 점을 찍을지 결정
x, y    (x + 1, y)

초기 bresenham알고리즘은 round 함수를 사용해 반올림.
round 함수의 비용이 크기에 mid-point 방식으로 향상.

일단 라인에 있는지 없는지 여부를 판단

점(Ax, Ay)과 점(Bx, By)가 있다고 할때, 2차원 좌표에서 표현시 x차이를 Width, y차이를 Height라 하자.
W(Width)  = Bx – Ax
H(Height) = By - Ay

H / W = (y - Ay) / (x - Ax)

양변에 W를 곱하고
H(x – Ax) = W(y – Ay)

좌항으로 옮기면 다음과 같은 식을 얻을 수 있게 된다.
H(x – Ax) - W(y – Ay)  = 0

나중에 float 연산을 피하기위해 2를 미리 곱하고 이를 F(x, y)라 칭하자.
F(x, y) = -2W(y – Ay) + 2H(x – Ax)

F(x, y)에 대해 다음과 같은 성질을 가짐을 알 수 있을것이다.
- F(x, y) < 0 - 라인 위
- F(x, y) = 0 - 라인에 있다
- F(x, y) > 0 - 라인 아래



F(x, y)에 대해 라인 위에 점을 찍을지 아래에 찍을지를 판별할 수 있으니 구해보는데 앞서,
 초기값을 mid-point를 주고, mid-point(x + 1, y + 1/2) 위에 점을 찍을지(x + 1, y + 1), 아래(x + 1, y)에 찍을지 방정식을 미리 계산해보자.

M(Mx, My) = (Ax + 1, Ay + 1/2)
F(Mx, My) = -2W((Ay + 1/2) – Ay) + 2H((Ax + 1) – Ax)
= -W + 2H


F(Ax + 1, Ay + 1/2) = 2H – W

* (x + 1, y)
F(Ax + 2, Ay + 1/2) - F(Ax + 1, Ay + 1/2)
= (4H - W) - (2H - W)
= 2H

F(Mx, My) += 2H


* (x + 1, y + 1)
F(Ax + 2, Ay + 1/2 + 1) - F(Ax + 1, Ay + 1/2)
= (-3W + 4H) - (2H – W)
= 2(H - W)

F(Mx, My) += (2H – 2W)


결과적으로 2H는 항상 더하고 mid-point를 검사하여 -2W를 할지 안할지를 결정만 하기만 하면 된다.

이를 의사코드로 풀어쓰면 다음과 같다.

H = dx = x1 - x0
W = dy = y1 - y0
y = y0

error = 0
for x in (x0 .. x1)
    set_pixel(x, y)
    error += 2H
    if (error > dx)
        y += 1
        error -= 2W


* Two step line drawing
 Bresenham알고리즘과 비등한 성능을 갖고 있습니
brian Wyvill 이라는 중국사람이 개발했습니다

bresenham은 2개의 픽셀 비교とw
two step line 은 3개의 픽셀 비교

기울기 : 0 ~ 0.5
   |   o|
ooo|oo  | oo
   |    |o

기울기 : 0.5 ~ 1

    o|    |
  o  |   o|  oo
o    |oo  |o

기울기 : 1 ~ 2

    o| o | o
  o  |o  | o
o    |o  |o

기울기 : 2 ~

o  | o | o
o  |o  | o
o  |o  |o


* Xiaolin Wu's line algorithm
 Bresenham 알고리즘이 직선을 그리는때 상당히 빠르지만, 이는 안티알리어스를 지원하지 않음.
 Bresenham보다 느리지만, 안티알리어스를 적용가능한 알고리즘을 개발
https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
