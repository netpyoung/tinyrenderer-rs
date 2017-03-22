let xs = vec![1.0, 6,0, 3.0];
xs.sort();

f32에 대해 Ord를 정의할 수 없기에 sort대신 sort_by를 써야한다.
